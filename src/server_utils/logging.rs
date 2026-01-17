use crate::server_utils::configuration::LogSettings;
use chrono::prelude::*;
use log::{log, Level, LevelFilter, SetLoggerError};
use std::fs::{read_dir, DirEntry, File};
use std::io::{Write};
use regex::Regex;

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
            let now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
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
        let today_string = today.format("%Y-%m-%d");
        let file_name = format!("log-{}.txt", today_string);
        let file_path = settings.path_string.clone() + file_name.as_str();
        let file = File::options()
            .append(true)
            .create(true)
            .open(file_path)
            .expect("Unable to open log file");

        Self::delete_outdated_log_files(&settings);

        Box::new(Logger {
            level: settings.max_level,
            file,
        })
    }

    pub fn delete_outdated_log_files(settings: &LogSettings) {
        let _today = Utc::now();
        let log_file_dir = match read_dir(settings.path_string.clone()) {
            Ok(files) => files,
            Err(e) => {
                log!(Level::Error, "Could not read directory: {}", e);
                return;
            }
        };
        let files_in_dir: Vec<DirEntry> = log_file_dir.filter_map(|result| result.ok()).collect();
        let files_to_remove: Vec<DirEntry> = Vec::new();
        let file_names: Vec<String> = files_in_dir
            .iter()
            .filter_map(|file| file.file_name().to_str().map(|s| s.to_string()))
            .collect();
        let log_file_reg_ex = Regex::new(LOG_FILE_REGEX).unwrap();
        let log_file_names: Vec<String> = file_names.iter().filter(|file_name| {
            let is_match = log_file_reg_ex.is_match(file_name);
            if !is_match {
                log!(Level::Warn, "File {} in log dir does not match regex", file_name);
            }
            is_match
        }).cloned().collect();
        //TODO get rid of unwraps
        let log_file_dates: Vec<DateTime<Utc>> = log_file_names.iter().map(|file_name| {
            let captures = log_file_reg_ex.captures(file_name.as_str()).unwrap();
            let year = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let month = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let day = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();

            Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap().into()
        }).collect();
        //TODO collect files to remove
        println!("Found log file dates: {:?}", log_file_dates);
        println!("Found files to remove: {:?}", files_to_remove);


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
