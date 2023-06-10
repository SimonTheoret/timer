use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const SEC_IN_MINUTE: f32 = 60.;
const MAX_MINUTES: f32 = 120.;

fn main() -> Result<(), eframe::Error> {
    let timer = Timer::new(50.);
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("Rust timer app", options, Box::new(|_cc| Box::new(timer)))
}
struct Timer {
    duration: Arc<Mutex<f32>>,
    widget_visible: Arc<Mutex<bool>>,
    init_duration: f32,
}

impl Timer {
    fn new(duration: f32) -> Self {
        Timer {
            duration: Arc::new(Mutex::new(duration)),
            widget_visible: Arc::new(Mutex::new(true)),
            init_duration: duration,
        }
    }
    fn start(&mut self, ctx: &egui::Context) {
        self.init_duration = self.duration.lock().unwrap().clone();
        let duration_clone = Arc::clone(&self.duration);
        let context = ctx.clone();
        self.set_widget_state_false();
        context.request_repaint();
        let handle = thread::spawn(move || {
            while *duration_clone.lock().unwrap() > 0. {
                {
                    *duration_clone.lock().unwrap() -= 1. / 60.; // lock inside its own scope as to
                                                                 // NOT freeze the ui
                }
                context.request_repaint();
                thread::sleep(Duration::new(1, 0));
            }
        });
        while handle.is_finished() {
            self.set_widget_state_true()
        }
    }
    fn widget_state(&self) -> bool {
        *self.widget_visible.lock().unwrap()
    }
    fn set_widget_state_true(&mut self) {
        *self.widget_visible.lock().unwrap() = true
    }
    fn set_widget_state_false(&mut self) {
        *self.widget_visible.lock().unwrap() = false
    }
}

impl eframe::App for Timer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let timeleft = self.duration.lock().unwrap().clone();
            let minutes = timeleft.clone().floor();
            let seconds = (timeleft.clone().fract() * SEC_IN_MINUTE).floor(); //TODO:
            // Make it better.
            let progress_bar = egui::widgets::ProgressBar::new(
                *self.duration.lock().unwrap() / self.init_duration,
            );
            let progress_bar_textual = progress_bar.text(format!("{minutes} : {seconds}"));
            ui.heading("Pomodoro");
            ui.add_visible(!self.widget_state(), progress_bar_textual);
            ui.add_visible(
                self.widget_state(),
                egui::Slider::new(&mut *self.duration.lock().unwrap(), 0.0..=MAX_MINUTES)
                    .text("Duration"),
            );
            ui.add_visible_ui(self.widget_state(), |ui| {
                if ui.button("+").clicked() {
                    *self.duration.lock().unwrap() += 1.;
                }
                if ui.button("-").clicked() {
                    *self.duration.lock().unwrap() -= 1.;
                }
                if ui.button("Start timer").clicked() {
                    self.start(ctx);
                }
            });
        });
    }
}
