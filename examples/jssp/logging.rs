use std::{path::{Path, PathBuf}, collections::HashMap};

use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger},
    encode::pattern::PatternEncoder,
};

pub fn init_logging(log_files: &HashMap<String, PathBuf>) -> Result<log4rs::Handle, log::SetLoggerError> {
    let log_pattern = String::from("[{l}] {m}{n}");
    let csv_log_pattern = String::from("{m}{n}");

    let stdout_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(&log_pattern)))
        .build();

    let mut cfg_builder = log4rs::Config::builder();

    // Register console appender
    cfg_builder = cfg_builder.appender(Appender::builder().build("main", Box::new(stdout_appender)));

    // Register appenders & loggers for given events
    if !log_files.is_empty() {
        let csv_encoder = Box::new(PatternEncoder::new(&csv_log_pattern));
        for (event_name, log_file) in log_files.iter() {
            let csv_appender = FileAppender::builder()
                .encoder(csv_encoder.clone())
                .append(false)
                .build(log_file)
                .unwrap();

            cfg_builder = cfg_builder
                .appender(Appender::builder().build(event_name, Box::new(csv_appender)))
                .logger(
                    Logger::builder()
                        .appender(event_name)
                        .additive(false)
                        .build(event_name, log::LevelFilter::Info)
                );
        }
    }

    let config = cfg_builder
        .build(
            log4rs::config::Root::builder()
                .appender("main")
                .build(log::LevelFilter::Trace),
        )
        .unwrap();
    // .appender(FileAppender::builder().encoder(Box::new(PatternEncoder::new(&csv_log_pattern))).build("log.txt"))

    log4rs::init_config(config)
}
