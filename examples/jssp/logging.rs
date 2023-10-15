use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger},
    encode::pattern::PatternEncoder,
};

#[derive(serde::Serialize, Clone, Debug)]
pub struct OutputData<'a> {
    pub solution_string: String,
    pub hash: String,
    pub fitness: usize,
    pub generation_count: usize,
    pub total_time: u128,
    pub chromosome: &'a [f64],
}

pub fn init_logging(
    event_log_files: &HashMap<String, PathBuf>,
    metadata_log_file: &PathBuf,
) -> Result<log4rs::Handle, log::SetLoggerError> {
    let log_pattern = String::from("[{l}] {m}{n}");
    let csv_log_pattern = String::from("{m}{n}");
    let csv_encoder = Box::new(PatternEncoder::new(&csv_log_pattern));

    let stdout_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(&log_pattern)))
        .build();

    let mut cfg_builder = log4rs::Config::builder();

    // Register console appender
    cfg_builder = cfg_builder.appender(Appender::builder().build("main", Box::new(stdout_appender)));

    // Register appenders & loggers for given events
    if !event_log_files.is_empty() {
        let csv_encoder = Box::new(PatternEncoder::new(&csv_log_pattern));
        for (event_name, log_file) in event_log_files.iter() {
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
                        .build(event_name, log::LevelFilter::Info),
                );
        }
    }

    let result_appender = FileAppender::builder()
        .encoder(csv_encoder)
        .append(false)
        .build(metadata_log_file)
        .unwrap();

    cfg_builder = cfg_builder
        .appender(Appender::builder().build("metadata_appender", Box::new(result_appender)))
        .logger(
            Logger::builder()
                .appender("metadata_appender")
                .additive(false)
                .build("metadata", log::LevelFilter::Info),
        );

    let config = cfg_builder
        .build(
            log4rs::config::Root::builder()
                .appender("main")
                .build(log::LevelFilter::Trace),
        )
        .unwrap();

    log4rs::init_config(config)
}
