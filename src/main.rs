use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{println, thread};

const SEC_IN_MINUTE: f32 = 60.;
const MAX_MINUTES: f32 = 120.;

fn main() -> Result<(), eframe::Error> {
    let mut timer = Timer::default();
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("Rust timer app", options, Box::new(|_cc| Box::new(timer)))
}
enum Durations {
    MutExlusive(Arc<Mutex<f32>>),
    FloatDuration(f32),
}
impl Durations {
    fn switch_type(self) -> Durations {
        match self {
            Durations::MutExlusive(inside) => Durations::FloatDuration(*inside.lock().unwrap()),
            Durations::FloatDuration(f32) => Durations::MutExlusive(Arc::new(Mutex::new(f32))),
        }
    }
}
struct Timer {
    duration: Durations,
}

impl Timer {
    fn start(&mut self) {
        todo!()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            duration: Durations::FloatDuration(50.),
        }
    }
}

impl eframe::App for Timer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pomodoro");
            ui.add(
                egui::Slider::new(&mut inside_duration.lock().unwrap(), 0.0..=MAX_MINUTES)
                    .text("Duration"),
            );
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
