use std::path::Path;

use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger},
    encode::pattern::PatternEncoder,
};

pub fn init_logging(log_file: Option<&Path>) -> Result<log4rs::Handle, log::SetLoggerError> {
    let log_pattern = String::from("[{l}] {m}{n}");
    let csv_log_pattern = String::from("{m}{n}");

    let stdout_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(&log_pattern)))
        .build();

    let config =
        log4rs::Config::builder().appender(Appender::builder().build("main", Box::new(stdout_appender)));

    let config = if let Some(log_file) = log_file {
        let csv_appender = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(&csv_log_pattern)))
            .append(false)
            .build(log_file)
            .unwrap();
        config
            .appender(Appender::builder().build("csv_appender", Box::new(csv_appender)))
            .logger(
                Logger::builder()
                    .appender("csv_appender")
                    .additive(false)
                    .build("csv", log::LevelFilter::Info),
            )
    } else {
        config
    };

    let config = config
        .build(
            log4rs::config::Root::builder()
                .appender("main")
                .build(log::LevelFilter::Trace),
        )
        .unwrap();
    // .appender(FileAppender::builder().encoder(Box::new(PatternEncoder::new(&csv_log_pattern))).build("log.txt"))

    log4rs::init_config(config)
}
