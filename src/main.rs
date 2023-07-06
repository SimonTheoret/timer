use eframe::egui;

mod logger;
mod guistate;
mod pomodoro;
mod timer;
mod state_object;

fn main() -> Result<(), eframe::Error> {

    let pomodoro = pomodoro::Pomodoro::new();
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(egui::vec2(240.0, 200.0)),
        ..Default::default()
    };
    eframe::run_native("Rust timer app", options, Box::new(|_cc| Box::new(pomodoro)))
}
