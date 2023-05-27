use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Logger},
    encode::pattern::PatternEncoder,
};

pub fn init_logging() -> Result<log4rs::Handle, log::SetLoggerError> {
    let log_pattern = String::from("[{l}] {m}{n}");

    let stdout_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(&log_pattern)))
        .build();

    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("main", Box::new(stdout_appender)))
        .logger(
            Logger::builder()
                .appender("main")
                .additive(false)
                .build("mainlog", log::LevelFilter::Info),
        )
        .build(
            log4rs::config::Root::builder()
                .appender("main")
                .build(log::LevelFilter::Trace),
        )
        .unwrap();

    log4rs::init_config(config)
}
