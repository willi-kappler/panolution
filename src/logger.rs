// External modules
use simplelog::{Config, TermLogger, WriteLogger, LogLevelFilter};
use log::LogLevel;
use chrono::Local;

// System modules
use std::fs::OpenOptions;

pub fn create_logger() {
    let dt = Local::now();
    let log_file_name = dt.format("panolution_%Y_%m_%d.log").to_string();

    let log_config = Config{
        time: Some(LogLevel::Warn),
        level: Some(LogLevel::Warn),
        target: Some(LogLevel::Warn),
        location: Some(LogLevel::Warn)
    };

    if let Ok(file) = OpenOptions::new().append(true).create(true).open(&log_file_name) {
        let _ = WriteLogger::init(LogLevelFilter::Info, log_config, file);
        info!("Log file '{}' opened succesfully", &log_file_name);
    } else {
        // Log file could not be created, use stdout instead
        let _ = TermLogger::init(LogLevelFilter::Info, log_config);
        warn!("Could not open log fle: '{}', using sdtout instead!", &log_file_name);
    }
}
