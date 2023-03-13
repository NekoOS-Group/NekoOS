
use crate::arch::dev::io::Stdout;
use core::fmt::Write;

pub fn print(args: core::fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::dev::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::dev::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            log::Level::Error => 31, // Red
            log::Level::Warn => 93,  // BrightYellow
            log::Level::Info => 34,  // Blue
            log::Level::Debug => 32, // Green
            log::Level::Trace => 90, // BrightBlack
        };
        crate::println!(
            "\u{1B}[{}m[{}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }
    fn flush(&self) {}
}

pub fn init() {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => log::LevelFilter::Error,
        Some("WARN") => log::LevelFilter::Warn,
        Some("INFO") => log::LevelFilter::Info,
        Some("DEBUG") => log::LevelFilter::Debug,
        Some("TRACE") => log::LevelFilter::Trace,
        _ => log::LevelFilter::Off,
    });
}
