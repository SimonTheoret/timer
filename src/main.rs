use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{println, thread};

const SEC_IN_MINUTE: f64 = 60.;
const MAX_MINUTES: f64 = 120.;

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
    MutExlusive(Arc<Mutex<f64>>),
    FloatDuration(f64),
}
impl Durations {
    fn go_type_float(self) -> Result<Durations, &'static str> {
        match self {
            Durations::MutExlusive(inside) => Ok(Durations::FloatDuration(*inside.lock().unwrap())),
            Durations::FloatDuration(float) => {
                Err("Is already of type Durations::FloatDuration(float)")
            }
            _ => Err("Wrong initial type"),
        }
    }
    fn go_type_mutex(self) -> Result<Durations, &'static str> {
        match self {
            Durations::MutExlusive(inside) => {
                Err("Is already of type Durations::MutExlusive(Arc<Mutex<float>>)")
            }
            Durations::FloatDuration(float) => Ok(Durations::MutExlusive(Arc::new(Mutex::new(float)))),
            _ => Err("Wrong initial type"),
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
    fn go_type_float(&mut self) {
        self.duration = self.duration.go_type_float().unwrap() // Using unwrap because error
        // handling is done in self.duration.go_type_float
    }
    fn go_type_mutex(&mut self) {
        self.duration = self.duration.go_type_mutex().unwrap() // Using unwrap because error
        // handling is done in self.duration.go_type_mutex
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
                egui::Slider::new(&mut self.duration, 0.0..=MAX_MINUTES)
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
