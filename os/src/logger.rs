use log::Level;

use crate::println;

pub struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    /// echo -e "\x1b[31mhello world\x1b[0m"
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            // Before realizing thread, it always be 0.
            let thread_id = 0;
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
                thread_id,
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
