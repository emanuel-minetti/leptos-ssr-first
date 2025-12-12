use std::fs::{read_dir, remove_file, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::SystemTime;
use chrono::Days;
use log::{Level, LevelFilter, SetLoggerError};
use chrono::prelude::*;
use crate::server_utils::configuration::LogSettings;

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

    fn flush(&self) { }
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

        // delete outdated
        let _filtered_log_files = read_dir(settings.path_string)
            .unwrap()
            .filter(move |entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .metadata()
                    .unwrap()
                    .created()
                    .unwrap()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    <= today
                    .checked_sub_days(Days::new(settings.days_to_keep))
                    .unwrap()
                    .timestamp()
                    .try_into()
                    .unwrap()
            })
            .for_each(move |file| remove_file::<PathBuf>(file.unwrap().path()).unwrap());

        Box::new(Logger {
            level: settings.max_level,
            file,
        })
    }

    pub fn init(config: LogSettings) -> Result<(), SetLoggerError> {
        let max_level: LevelFilter = config.max_level.to_level_filter();
        log::set_boxed_logger(Self::new(config)).map(|()| log::set_max_level(max_level))
    }
}