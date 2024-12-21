use colored::Colorize;
use log::{Level, LevelFilter, Metadata, Record};

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => {
                    anstream::eprintln!("{}{} {}", "error".bold().red(), ":".bold(), record.args())
                }
                Level::Warn => anstream::eprintln!(
                    "{}{} {}",
                    "warn".bold().yellow(),
                    ":".bold(),
                    record.args()
                ),
                _ => anstream::println!("{}", record.args()),
            }
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&SimpleLogger)
        .map(|_| log::set_max_level(LevelFilter::Info))
        .expect("failed to init logging");
}
