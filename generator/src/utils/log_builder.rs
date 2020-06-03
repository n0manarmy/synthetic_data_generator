use log::{debug, error, info, warn, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::Handle;

use crate::args_helpers::argument_helper::LogLevel;

pub struct LogBuilder {}

impl LogBuilder {

    /// Builds our logger based on a programmatic implementation. This reduces the required
    /// config files needed to run the application.
    pub fn build_logger(log_level: &LogLevel) -> Handle {
        let stdout = ConsoleAppender::builder().build();
        let appender = Appender::builder().build("stdout", Box::new(stdout));
    
        let mut logger = Logger::builder().build("app::backend::db", LevelFilter::Info);
        let mut builder = Root::builder().appender("stdout").build(LevelFilter::Info);
    
        match log_level {
            LogLevel::Debug => {
                logger = Logger::builder().build("app::backend::db", LevelFilter::Debug);
                builder = Root::builder().appender("stdout").build(LevelFilter::Debug);
                debug!("Debug log enabled");
            }
            LogLevel::Info => info!("Info log enabled"),
            LogLevel::Warn => {
                logger = Logger::builder().build("app::backend::db", LevelFilter::Warn);
                builder = Root::builder().appender("stdout").build(LevelFilter::Warn);
                warn!("Warn log enabled");
                
            }
            LogLevel::Error => {
                logger = Logger::builder().build("app::backend::db", LevelFilter::Error);
                builder = Root::builder().appender("stdout").build(LevelFilter::Error);
                error!("Error log enabled");
            }
        }
        let config = Config::builder().appender(appender).logger(logger).build(builder).unwrap();
    
        log4rs::init_config(config).unwrap()
    }
}