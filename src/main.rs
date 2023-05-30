use chrono::Duration;
use eframe::egui;
use std::todo;
use timer::{Guard, Timer};

const SEC_IN_MINUTE: i32 = 60;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Rust timer app",
        options,
        Box::new(|_cc| Box::<TimerWrapper>::default()),
    )
}

struct TimerWrapper {
    timer: Timer,
    duration: i32,
}

impl TimerWrapper {
    fn new(timer: Timer, duration: i32, elapsed_time: i32, remaining_time: i32) -> TimerWrapper {
        TimerWrapper {
            timer,
            duration,
        }
    }
    fn start_timer(&self) {
        todo!()
    }
    fn update_logger(&self, log: Log) {
        todo!()
    }
    fn end_timer(&self) {
        todo!()
    }
    fn update_all(&self) {
        todo!()
    }
    fn schedule_with_delay_update(&self, delay: Duration) -> Guard {
        // self.timer.schedule_with_delay(delay, closure here)
        todo!()
    }
    fn schedule_repeating(&self, delay: Duration) -> Guard {
        // self.timer.schedule_with_delay(Duration, cb)
        todo!()
    }
}

impl Default for TimerWrapper {
    fn default() -> Self {
        Self {
            timer: Timer::new(),
            duration: 50,
        }
    }
}

impl eframe::App for TimerWrapper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pomodoro");
            ui.add(egui::Slider::new(&mut self.duration, 0..=120).text("minutes"));
            ui.vertical_centered_justified(|ui| {

                if ui.button("+").clicked() {
                    self.duration += 1;
                }
                if ui.button("-").clicked() {
                    self.duration -= 1;
                }
            });
        });
    }
}

struct Log {
    path_to_write: String,
    frequency: i32, //TODO Make better
}

impl Default for Log {
    fn default() -> Self {
        todo!()
    }
}
