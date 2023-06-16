use eframe::egui;

mod timer;
mod logger;

fn main() -> Result<(), eframe::Error> {
    let timer = timer::Timer::new(50.);
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(egui::vec2(240.0, 200.0)),
        ..Default::default()
    };
    eframe::run_native("Rust timer app", options, Box::new(|_cc| Box::new(timer)))
}
