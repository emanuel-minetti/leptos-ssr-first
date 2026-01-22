use crate::server_utils::configuration::LogSettings;
use chrono::prelude::*;
use chrono::LocalResult;
use log::{log, Level, LevelFilter, SetLoggerError};
use regex::Regex;
use std::collections::HashMap;
use std::fs::{read_dir, DirEntry, File};
use std::io::Write;

const LOG_ENTRY_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";
const LOG_FILE_NAME_DATE_FORMAT: &str = "%Y-%m-%d";
const LOG_FILE_REGEX: &str = r"^log-(\d{4})-(\d{2})-(\d{2})\.txt$";

pub struct Logger {
    level: Level,
    file: File,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let file = &mut self.file.try_clone().unwrap();
            let now = Utc::now().format(LOG_ENTRY_DATE_FORMAT);
            println!("{} [{}]: {}", now, record.level(), record.args());
            writeln!(file, "{} [{}]: {}", now, record.level(), record.args())
                .expect("Could not write to log file");
            file.flush().unwrap();
        }
    }

    fn flush(&self) {}
}

impl Logger {
    pub fn new(settings: LogSettings) -> Box<Self> {
        let today = Utc::now();
        let today_string = today.format(LOG_FILE_NAME_DATE_FORMAT);
        // TODO consider using `const-format` crate for using a CONST here
        let file_name = format!("log-{}.txt", today_string);
        let file_path = settings.path_string.clone() + file_name.as_str();
        let file = File::options()
            .append(true)
            .create(true)
            .open(file_path)
            .expect("Unable to open log file");
        Self::delete_outdated_log_files(&settings, true);

        Box::new(Logger {
            level: settings.max_level,
            file,
        })
    }

    pub fn delete_outdated_log_files(settings: &LogSettings, running_on_startup: bool) {
        let _today = Utc::now();
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
        // TODO collect log files to remove

        println!("Found files: {:?}", files_in_dir_map);

        // let _filtered_log_files = read_dir(settings.path_string.clone())
        //     .unwrap()
        //     .filter(move |entry| {
        //         entry
        //             .as_ref()
        //             .unwrap()
        //             .metadata()
        //             .unwrap()
        //             .created()
        //             .unwrap()
        //             .duration_since(SystemTime::UNIX_EPOCH)
        //             .unwrap()
        //             .as_secs()
        //             <= today
        //             .checked_sub_days(Days::new(settings.days_to_keep))
        //             .unwrap()
        //             .timestamp()
        //             .try_into()
        //             .unwrap()
        //     })
        //     .for_each(move |file| remove_file::<PathBuf>(file.unwrap().path()).unwrap());
    }

    pub fn init(config: LogSettings) -> Result<(), SetLoggerError> {
        let max_level: LevelFilter = config.max_level.to_level_filter();
        log::set_boxed_logger(Self::new(config)).map(|()| log::set_max_level(max_level))
    }
}
