use chrono::{DateTime, Local};
use csv::Writer;
use directories::BaseDirs;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::path::PathBuf;

pub struct Logger {
    writer: Result<Writer<File>, Box<dyn Error>>,
    start_time: DateTime<Local>,
}

impl Logger {
    fn get_local_dir() -> Result<PathBuf, &'static str> {
        if let Some(base_dirs) = BaseDirs::new() {
            Ok(base_dirs.data_local_dir().join("pomodoro"))
        } else {
            Err("No home directory found! Make sure you have a home directory on your OS.")
        }
    }
    fn create_pomodoro_dir() -> Result<Box<PathBuf>, Box<dyn Error>> {
        let result = Logger::get_local_dir()?;
        let path = result.as_path();
        create_dir_all(path)?;
        Ok(Box::new(result))
    }
    fn get_csv_writer() -> Result<Writer<File>, Box<dyn Error>> {
        let path = Logger::create_pomodoro_dir();
        Ok(Writer::from_path(path?.join("data.csv").as_path())?)
    }
    pub fn new(start_time: DateTime<Local>) -> Logger {
        Logger {
            writer: Logger::get_csv_writer(),
            start_time,
        }
    }
    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let writer = self.writer.as_ref(); // BUG: plz fix
        let start_time = self.start_time;
        let now = Local::now();
        let duration = now - start_time;
        let (start_time, now, duration) = (
            start_time.to_rfc3339(),
            now.to_rfc3339(),
            duration.to_string(),
        );
        if let Ok(csv_writer) = writer {
            csv_writer.write_record(&[start_time, now, duration])?;
        }
        Ok(())
    }
}
