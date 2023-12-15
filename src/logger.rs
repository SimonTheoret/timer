use chrono::{DateTime, Local};
use csv::Writer;
use directories::BaseDirs;
use std::fs::{create_dir_all, File, OpenOptions};
use std::path::PathBuf;

pub struct Logger {
    writer: Writer<File>,
    start_time: DateTime<Local>,
}

impl Logger {
    pub fn new() -> Option<Logger> {
        let start_time = Local::now();
        let writer = match Logger::get_csv_writer() {
            None => return None,
            Some(writer) => writer,
        };
        Some(Logger { writer, start_time })
    }
    fn get_local_dir() -> Option<PathBuf> {
        if let Some(base_dirs) = BaseDirs::new() {
            Some(base_dirs.data_local_dir().join("pomodoro"))
        } else {
            eprintln!("No home directory found! Make sure you have a home directory on your OS.");
            None
        }
    }

    fn create_pomodoro_dir() -> Option<PathBuf> {
        let directory = match Logger::get_local_dir() {
            Some(dir) => dir,
            _ => return None,
        };
        match create_dir_all(directory.as_path()) {
            Ok(()) => (),
            Err(_) => eprintln!("Could not create the directories"),
        };
        let path = directory.join("data.csv");
        Some(path)
    }
    fn get_csv_writer() -> Option<Writer<File>> {
        Logger::create_pomodoro_dir()?;
        let path_buf = Logger::create_pomodoro_dir().unwrap();
        let file = OpenOptions::new().create(true).append(true).open(path_buf);
        if let Err(err) = file {
            eprintln!("{err}");
             None
        } else {
            Some(Writer::from_writer(file.unwrap()))
        }

        // let file2 = match file {
        //     Ok(file) => file,
        //     Err(err) => return None,
        // };
    }

    pub fn write(&mut self) {
        let start_time = self.start_time;
        let now = Local::now();
        let duration = now - start_time;
        let (start_time, now, duration) = (
            start_time.to_rfc3339(),
            now.to_rfc3339(),
            duration.to_string(),
        );
        match self.writer.write_record(&[start_time, now, duration]) {
            Ok(()) => println!("writing !"),
            Err(_) => eprintln!("Cannot write. Why? I'm not sure..."),
        }
        self.writer.flush();
    }
}
pub fn conditional_write(logger_option: &mut Option<Logger>) {
    match logger_option {
        Some(logger) => logger.write(),
        None => (),
    }
}
