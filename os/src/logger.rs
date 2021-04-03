use crate::println;
use log::{Level, Metadata, Record};

// Copied from: https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html

pub struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                log::Level::Error => println!("\x1b[31m[Error][0] {}\x1b[0m", record.args()),
                log::Level::Warn => println!("\x1b[93m[Warn][0] {}\x1b[0m", record.args()),
                log::Level::Info => println!("\x1b[34m[Info][0] {}\x1b[0m", record.args()),
                log::Level::Debug => println!("\x1b[32m[Debug][0] {}\x1b[0m", record.args()),
                log::Level::Trace => println!("\x1b[90m[Trace][0] {}\x1b[0m", record.args()),
            }
        }
    }

    fn flush(&self) {}
}
