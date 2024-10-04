use log4rs::append::Append;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;

const LOG_PATTERN: &str = "{d} {l} - {m}{n}";

pub fn init_logger(log_file: &str) {
    let appender: Box<dyn Append>;

    if log_file.is_empty() {
        appender = create_console_appender();
    } else {
        appender = create_file_appender(log_file);
    }

    let config = Config::builder()
        .appender(Appender::builder().build("app_appender", appender))
        .build(Root::builder()
            .appender("app_appender")
            .build(LevelFilter::Info)).unwrap();

    log4rs::init_config(config).unwrap();
}

fn create_file_appender(log_file: &str) -> Box<FileAppender> {
    Box::from(FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .build(log_file)
        .unwrap())
}

fn create_console_appender() -> Box<ConsoleAppender> {
    Box::from(ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .build())
}