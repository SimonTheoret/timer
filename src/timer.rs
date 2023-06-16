use chrono::Local;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::logger::Logger;

const SEC_IN_MINUTE: f32 = 60.;
const MAX_MINUTES: f32 = 120.;

pub struct Timer {
    duration: Arc<Mutex<f32>>,
    widget_visible: Arc<Mutex<bool>>,
    init_duration: f32,
    stop_thread: Arc<Mutex<bool>>,
    logger: Option<Logger>,
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Timer {
            duration: Arc::new(Mutex::new(duration)),
            widget_visible: Arc::new(Mutex::new(true)),
            init_duration: duration,
            stop_thread: Arc::new(Mutex::new(false)),
            logger: Option::None,
        }
    }
    fn start(&mut self, ctx: &egui::Context) {
        self.init_duration = self.duration.lock().unwrap().clone();
        *self.stop_thread.lock().unwrap() = false;
        self.logger = Option::Some(Logger::new(Local::now()));
        let duration_clone = Arc::clone(&self.duration);
        let stop_thread_clone = Arc::clone(&self.stop_thread);
        let widget_visible_clone = Arc::clone(&self.widget_visible);
        let context = ctx.clone();
        self.set_widget_state_false();
        context.request_repaint();
        let sleep_duration = Duration::new(1, 0);
        let _handle = thread::spawn(move || {
            // Should we remove handle as a name?
            while *duration_clone.lock().unwrap() >= 1. / 60. {
                {
                    *duration_clone.lock().unwrap() -= 1. / 60.; // lock inside its own scope as to
                                                                 // NOT freeze the ui
                }
                context.request_repaint();
                if *stop_thread_clone.lock().unwrap() == true {
                    *widget_visible_clone.lock().unwrap() = true;
                    return;
                }
                thread::sleep(sleep_duration);
            }

            *stop_thread_clone.lock().unwrap() = true;
            *widget_visible_clone.lock().unwrap() = true;
            context.request_repaint();
        });
        // self.set_widget_state_true();
        // self.stop(); // used for ending of method
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
    fn convert_to_secs(&mut self, time_left: &f32) -> f32 {
        (time_left.fract() * SEC_IN_MINUTE).floor()
    }
    fn create_progress_bar(&mut self, time_left: &f32) -> egui::widgets::ProgressBar {
        let progress_bar = egui::widgets::ProgressBar::new(time_left / self.init_duration);
        let (minutes, seconds) = (time_left.floor(), self.convert_to_secs(&time_left));
        progress_bar.text(format!("{}:{:0>2}", minutes, seconds))
    }
    fn get_duration(&mut self) -> f32 {
        *(&self.duration).lock().unwrap()
    }
    fn logger_is_on(&self) -> bool {
        if let Option::Some(_) = self.logger {
            true
        } else {
            false
        }
    }
}
fn stop(timer: &Timer)-> Result<(), Box<dyn Error>> {
    let mut thread_on_pause = timer.stop_thread.lock().unwrap();
    if timer.logger_is_on() {
        timer.logger.as_ref().unwrap().write()?
    }
    *thread_on_pause = true;
    Ok(())
}

impl eframe::App for Timer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let duration_clone = self.get_duration();
            ui.heading("Pomodoro");
            ui.add_visible(
                !self.widget_state(),
                self.create_progress_bar(&duration_clone),
            );
            ui.add_visible(
                self.widget_state(),
                egui::Slider::new(&mut *self.duration.lock().unwrap(), 0.0..=MAX_MINUTES)
                    .text("Duration"),
            );
            ui.vertical_centered_justified(|ui| {
                ui.add_visible_ui(self.widget_state(), |ui| {
                    if ui.button("+").clicked() {
                        *self.duration.lock().unwrap() += 1.;
                    }
                    if ui.button("-").clicked() {
                        *self.duration.lock().unwrap() -= 1.;
                    }
                })
            });
            ui.vertical_centered_justified(|ui| {
                ui.add_visible_ui(self.widget_state(), |ui| {
                    if ui.button("Start timer").clicked() {
                        self.start(ctx);
                    }
                });
                ui.add_visible_ui(!self.widget_state(), |ui| {
                    if ui.button("Stop timer").clicked() {
                        stop(&self);
                    }
                })
            });
        });
    }
}
