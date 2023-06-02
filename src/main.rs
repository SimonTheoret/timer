use eframe::egui;
use std::sync::mpsc::channel;
use std::{thread, println};
use std::time::Duration;

const SEC_IN_MINUTE: f32 = 60.;
const MAX_MINUTES: f32 = 120.;

fn main() -> Result<(), eframe::Error> {
    let timer = Timer::default();
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("Rust timer app", options, Box::new(|_cc| Box::new(timer)))
}
struct Timer {
    duration: f32,
}

impl Timer {
    fn start(&mut self) {
        while self.duration * SEC_IN_MINUTE > 0. {
            thread::sleep(Duration::from_secs(1));
            self.duration -= 1./SEC_IN_MINUTE;
            println!("{}", self.duration);
        }
        println!("Done")
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self { duration: 50. }
    }
}

impl eframe::App for Timer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pomodoro");
            ui.add(egui::Slider::new(&mut self.duration, 0.0..=MAX_MINUTES).text("Duration"));
            ui.vertical_centered_justified(|ui| {
                if ui.button("+").clicked() {
                    self.duration += 1.;
                }
                if ui.button("-").clicked() {
                    self.duration -= 1.;
                }
                if ui.button("Start timer").clicked() {
                    self.start();
                }
            });
        });
    }
}
