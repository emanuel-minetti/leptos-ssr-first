use crate::server_utils::configuration::LogSettings;
use chrono::prelude::*;
use chrono::{Days, LocalResult};
use log::{log, Level, LevelFilter, SetLoggerError};
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::{read_dir, remove_file, DirEntry, File};
use std::io::Write;
use std::sync::{Mutex, OnceLock};

const LOG_ENTRY_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";
const LOG_FILE_NAME_DATE_FORMAT: &str = "%Y-%m-%d";
const LOG_FILE_REGEX: &str = r"^log-(\d{4})-(\d{2})-(\d{2})\.txt$";
const LOG_FILE_FORMAT: &str = "log-{}.txt";

pub struct Logger {
    level: Level,
    path: String,
    file: Mutex<File>,
    env: String,
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let file = &mut self.file.try_lock().expect("Couldn't lock log file");
            let now = Utc::now().format(LOG_ENTRY_DATE_FORMAT);
            writeln!(
                file,
                "{} [{}]: ({}) {}",
                now,
                record.level(),
                record.target(),
                record.args()
            )
                .expect("Could not write to log file");
            if self.env == "DEV" {
                println!(
                    "{} [{}]: ({}) {}",
                    now,
                    record.level(),
                    record.target(),
                    record.args()
                );
            }

            file.flush().unwrap();
        }
    }

    fn flush(&self) {}
}

impl Logger {
    pub async fn set_new_logfile() {
        let this = LOGGER.get().expect("LOGGER not initialized");
        let mut file = this.file.lock().expect("Couldn't lock log file");
        *file = Self::open_file(this.path.clone());
    }

    pub async fn delete_outdated_log_files(settings: &LogSettings, running_on_startup: bool) {
        let log_file_reg_ex = Regex::new(LOG_FILE_REGEX).unwrap();
        let log_file_dir = match read_dir(settings.path_string.clone()) {
            Ok(files) => files,
            Err(e) => {
                if running_on_startup {
                    log!(Level::Error, "Could not read directory: {}", e);
                } else {
                    println!("Error: Could not read directory: {}", e)
                }
                return;
            }
        };
        let files_in_dir_vec: Vec<DirEntry> =
            log_file_dir.filter_map(|result| result.ok()).collect();
        let mut files_in_dir_map: HashMap<String, Option<DateTime<Utc>>> = HashMap::new();
        // collect readable dir entries
        for dir_entry in files_in_dir_vec {
            match dir_entry.file_name().to_str() {
                Some(file_name) => {
                    files_in_dir_map.insert(file_name.to_string(), None);
                }
                None => {}
            }
        }
        // collect log files
        for (file_name, _) in &files_in_dir_map.clone() {
            if log_file_reg_ex.is_match(file_name.as_str()) {
                // this `unwrap` is approved because we know we have a match
                let capture = log_file_reg_ex.captures(file_name.as_str()).unwrap();
                let year = match capture.get(1).unwrap().as_str().parse::<i32>() {
                    Ok(x) => {
                        if x > 2000 && x < 3000 {
                            x
                        } else {
                            if !running_on_startup {
                                log!(
                                    Level::Error,
                                    "In plausible value for year in log file {}",
                                    file_name
                                );
                            } else {
                                println!(
                                    "Error: In plausible value for year in log file {}",
                                    file_name
                                );
                            }
                            return;
                        }
                    }
                    Err(_) => {
                        if !running_on_startup {
                            log!(
                                Level::Error,
                                "Couldn't parse year in log file {}",
                                file_name
                            );
                        } else {
                            println!("Error: Couldn't parse year in log file {}", file_name);
                        }
                        return;
                    }
                };
                let month = match capture.get(2).unwrap().as_str().parse::<u32>() {
                    Ok(x) => x,
                    Err(_) => {
                        if !running_on_startup {
                            log!(
                                Level::Error,
                                "Couldn't parse month in log file {}",
                                file_name
                            );
                        } else {
                            println!("Error: Couldn't parse month in log file {}", file_name);
                        }
                        return;
                    }
                };
                let day = match capture.get(3).unwrap().as_str().parse::<u32>() {
                    Ok(x) => x,
                    Err(_) => {
                        if !running_on_startup {
                            log!(Level::Error, "Couldn't parse day in log file {}", file_name);
                        } else {
                            println!("Error: Couldn't parse day in log file {}", file_name);
                        }
                        return;
                    }
                };
                let date = match Utc.with_ymd_and_hms(year, month, day, 0, 0, 0) {
                    LocalResult::Single(date) => date,
                    _ => {
                        if !running_on_startup {
                            log!(
                                Level::Error,
                                "Could not parse date in log file {}",
                                file_name
                            );
                        } else {
                            println!("Error: Couldn't parse date in log file {}", file_name);
                        }
                        return;
                    }
                };
                files_in_dir_map.insert(file_name.to_string(), Some(date));
            } else {
                if !running_on_startup {
                    log!(
                        Level::Warn,
                        "File {} in log dir does not match regex",
                        file_name
                    );
                } else {
                    println!(
                        "Warning: File {} in log dir does not match regex",
                        file_name
                    );
                }
            }
        }
        // collect log files to remove
        for (file_name, date_option) in &files_in_dir_map.clone() {
            if date_option.is_some() {
                let today = Utc::now();
                let file_date = date_option.unwrap();
                if file_date.checked_add_days(Days::new(settings.days_to_keep)) > today.into() {
                    files_in_dir_map.insert(file_name.to_string(), None);
                }
            }
        }
        // remove outdated log files
        for (file_name, date_option) in &files_in_dir_map.clone() {
            if date_option.is_some() {
                let full_file_path = settings.path_string.clone() + file_name.as_str();
                match remove_file(full_file_path) {
                    Ok(_) => {
                        if !running_on_startup {
                            log!(Level::Info, "Deleted outdated log file {}", file_name);
                        } else {
                            println!("Info: Deleted outdated log file {}", file_name);
                        }
                    }
                    Err(_) => {
                        if !running_on_startup {
                            log!(
                                Level::Error,
                                "Couldn't remove outdated log file {}",
                                file_name
                            );
                        } else {
                            println!("Error: Cannot remove outdated log file {}", file_name);
                        }
                    }
                };
            }
        }
        if !running_on_startup {
            log!(Level::Debug, "cleaned up logfiles");
        }
    }

    pub async fn init(config: LogSettings) -> Result<(), SetLoggerError> {
        let max_level: LevelFilter = config.max_level.to_level_filter();
        log::set_logger(Self::new(config).await).map(|()| log::set_max_level(max_level))
    }

    async fn new(settings: LogSettings) -> &'static Self {
        let file = Self::open_file(settings.path_string.clone());
        Self::delete_outdated_log_files(&settings, true).await;

        LOGGER.get_or_init(|| Logger {
            level: settings.max_level,
            path: settings.path_string,
            file: Mutex::new(file),
            env: env::var("ENV").unwrap_or_else(|_| "DEV".to_string()),
        })
    }

    fn open_file(path: String) -> File {
        let file_path = Self::new_filename(path);
        let file = File::options()
            .append(true)
            .create(true)
            .open(file_path)
            .expect("Unable to open log file");

        file
    }

    fn new_filename(path: String) -> String {
        let today = Utc::now();
        let today_string = today.format(LOG_FILE_NAME_DATE_FORMAT);
        let file_name = LOG_FILE_FORMAT
            .to_string()
            .replace("{}", today_string.to_string().as_str());

        path.clone() + file_name.as_str()
    }
}
