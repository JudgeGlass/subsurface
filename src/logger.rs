use log::{LogRecord, LogMetadata, Log, set_logger, SetLoggerError, LogLevelFilter};
use std::io::prelude::*;
use std::fs::File;
use std::sync::Mutex;
use time;

struct SimpleLogger {
    f: Mutex<File>,
}

impl Log for SimpleLogger {
    fn enabled(&self, _: &LogMetadata) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let mut f = self.f.lock().unwrap();
            write!(f,
                   "{} {} - {}\n",
                   time::strftime("%FT%TZ", &time::now_utc()).unwrap(),
                   record.level(),
                   record.args())
                .unwrap();
        }
    }
}

impl SimpleLogger {
    fn new() -> SimpleLogger {
        SimpleLogger { f: Mutex::new(File::create("subsurface.log").unwrap()) }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Debug);
        Box::new(SimpleLogger::new())
    })
}
