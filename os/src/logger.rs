//! Logger using [`crate::println!()`]

use log::Level;

use crate::println;

/// Logger using [`crate::println!()`]
pub struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let color_code = match record.level() {
                Level::Error => 31,
                Level::Warn => 93,
                Level::Info => 34,
                Level::Debug => 32,
                Level::Trace => 90,
            };
            println!(
                "\x1b[{}m[{}][{}] {}\x1b[0m",
                color_code,
                record.level(),
                // S-mode
                "S",
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

/// Init logger with env.
pub fn init() {
    static LOGGER: MyLogger = MyLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => log::LevelFilter::Error,
        Some("WARN") => log::LevelFilter::Warn,
        Some("INFO") => log::LevelFilter::Info,
        Some("DEBUG") => log::LevelFilter::Debug,
        Some("TRACE") => log::LevelFilter::Trace,
        _ => log::LevelFilter::Trace,
    });
}
