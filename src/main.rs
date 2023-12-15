use directories::BaseDirs;
use eframe::egui;
use log::LevelFilter::Error as log_error;
use std::error::Error;

mod guistate;
mod logger;
mod pomodoro;
mod state_object;
mod timer;

fn main() -> Result<(), Box<dyn Error>> {
    let bd = BaseDirs::new().ok_or("Something is wrong with data local directory")?;
    let log_path = bd.data_local_dir().join("pomodoro.log");

    let _logging = simple_logging::log_to_file(log_path, log_error);

    let pomodoro = pomodoro::Pomodoro::new();
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(egui::vec2(240.0, 200.0)),
        ..Default::default()
    };
    let _eframe_run = eframe::run_native(
        "Rust timer app",
        options,
        Box::new(|_cc| Box::new(pomodoro)),
    );
    Ok(())
}
