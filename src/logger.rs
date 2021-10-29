use log::{Level, Metadata, Record};

pub static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

pub struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) && record.target().starts_with("docu") {
            println!("{}", record.args());
        }
    }

    fn flush(&self) {}
}
