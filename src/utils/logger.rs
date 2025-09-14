//*           Logger          *//
//*   Developer: Urban Egor   *//
//*   Version: 1.3.14 r       *//



use std::fs::{OpenOptions, create_dir_all};
use std::io::Write; 
use std::sync::{mpsc, Arc};
use std::thread;
use chrono::Local;
use once_cell::sync::Lazy;
use std::path::Path;



#[derive(Clone, Copy)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}



struct LoggerInner {
    sender: mpsc::Sender<(LogLevel, String)>,
}



pub struct Logger {
    inner: Arc<LoggerInner>,
}



pub static LOGGER: Lazy<Logger> = Lazy::new(|| Logger::new());



impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::DEBUG => "DEBUG",
            LogLevel::INFO => "INFO",
            LogLevel::WARN => "WARN",
            LogLevel::ERROR => "ERROR",
        }
    }
}



impl Logger {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel::<(LogLevel, String)>();
        let log_dir = Path::new("logs");

        if !log_dir.exists() {
            if let Err(e) = create_dir_all(log_dir) {
                eprintln!("[X][ERROR][FATAL] Cant create logs dir: {}", e);
            }
        }

        // Log  thread
        thread::spawn(move || {
            while let Ok((level, msg)) = rx.recv() {
                let now = Local::now();
                let date_str = now.format("%Y-%m-%d").to_string();
                let datetime_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
                let full_msg = format!("[{}][{}] {}", datetime_str, level.as_str(), msg);

                // Terminal out
                println!("{}", full_msg);

                // File out
                let file_path = log_dir.join(format!("{}.log", date_str));
                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&file_path)
                {
                    let _ = writeln!(file, "{}", full_msg);
                }
            }
        });

        Logger {
            inner: Arc::new(LoggerInner { sender: tx }),
        }
    }


    fn log(&self, level: LogLevel, msg: &str) {
        let _ = self.inner.sender.send((level, msg.to_string()));
    }


    pub fn debug(&self, msg: &str) { self.log(LogLevel::DEBUG, msg); }
    pub fn info(&self, msg: &str)  { self.log(LogLevel::INFO, msg); }
    pub fn warn(&self, msg: &str)  { self.log(LogLevel::WARN, msg); }
    pub fn error(&self, msg: &str) { self.log(LogLevel::ERROR, msg); }
}
