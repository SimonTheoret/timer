use crate::guistate::GuiState;
use crate::logger::{conditional_write, Logger};
use crate::state_object::StateObject;
use crate::timer::Timer;
use notify_rust::Notification;
use soloud::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

const SEC_IN_MINUTE: f32 = 60.;
const MAX_MINUTES: f32 = 120.;

pub struct Pomodoro {
    state: StateObject,
    init_duration: f32,
    rx_state: Option<Receiver<StateObject>>,
    tx_counting_down: Option<Sender<bool>>,
    logger: Option<Logger>,
}
impl Pomodoro {
    pub fn new() -> Pomodoro {
        Pomodoro {
            state: StateObject {
                duration: 50.,
                gui_state: GuiState::OptionMenu,
            },
            init_duration: 50.,
            rx_state: None,
            tx_counting_down: None,
            logger: None,
        }
    }
    fn is_option_menu_state(&self) -> bool {
        if self.state.gui_state == GuiState::OptionMenu {
            return true;
        };
        false
    }
    fn is_counting_down_state(&self) -> bool {
        if self.state.gui_state == GuiState::CountingDown {
            return true;
        };
        false
    }
    fn create_progress_bar(&mut self, time_left: &f32) -> egui::widgets::ProgressBar {
        let progress_bar = egui::widgets::ProgressBar::new(time_left / self.init_duration);
        let (minutes, seconds) = (time_left.floor(), self.convert_to_secs(time_left));
        progress_bar.text(format!("{}:{:0>2}", minutes, seconds))
    }
    fn convert_to_secs(&mut self, time_left: &f32) -> f32 {
        (time_left.fract() * SEC_IN_MINUTE).floor()
    }
    fn generate_state_obj(&self) -> StateObject {
        self.state.clone()
    }
    fn create_channels(&mut self) -> (Sender<StateObject>, Receiver<bool>) {
        let (tx_state, rx_state) = channel::<StateObject>();
        let (tx_counting_down, rx_counting_down) = channel::<bool>();
        self.rx_state = Some(rx_state);
        self.tx_counting_down = Some(tx_counting_down);
        (tx_state, rx_counting_down)
    }
    pub fn start(&mut self) {
        let given_state = self.generate_state_obj();
        let (tx_state, rx_counting_down) = self.create_channels();
        let mut timer = Timer::new(given_state, rx_counting_down, tx_state);
        self.init_duration = self.state.duration;
        self.state.gui_state = GuiState::CountingDown;
        let _handle = thread::spawn(move || {
            timer.count_down();
        });
    }
    #[allow(unused_must_use)]
    pub fn stop(&mut self) {
        self.tx_counting_down.as_ref().unwrap().send(false);
        match Notification::new()
            .summary("Rust timer:")
            .body("Working period is done!")
            .show()
        {
            Ok(_) => (),
            Err(_) => (),
        }
        let sl = Soloud::default().unwrap();
        let mut wav = audio::Wav::default();
        wav.load_mem(include_bytes!("./mixkit-interface-hint-notification-911.wav")).unwrap();
        sl.play(&wav);
    }
    fn update_gui_state(&mut self) {
        if self.rx_state.is_some() {
            if let Ok(state_obj) = self.rx_state.as_ref().unwrap().try_recv() {
                self.state = state_obj;
            }
        }
    }
    fn kill_logger(&mut self) {
        self.logger = Option::None;
    }
}
impl eframe::App for Pomodoro {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_gui_state();
            if self.state.duration <= 1./60. && self.state.gui_state == GuiState::CountingDown {
                conditional_write(&mut self.logger);
                self.kill_logger();
            }; //TODO: Do not call conditional_write() every time. BUG: <--
            ctx.request_repaint();
            let duration = self.state.duration;
            ui.heading("Pomodoro");
            ui.add_visible(
                self.is_counting_down_state(),
                self.create_progress_bar(&duration),
            );
            ui.add_visible(
                self.is_option_menu_state(),
                egui::Slider::new(&mut self.state.duration, 0.0..=MAX_MINUTES).text("Duration"),
            );
            ui.vertical_centered_justified(|ui| {
                ui.add_visible_ui(self.is_option_menu_state(), |ui| {
                    if ui.button("+").clicked() {
                        self.state.duration += 1.;
                    }
                    if ui.button("-").clicked() {
                        self.state.duration -= 1.;
                    }
                })
            });
            ui.vertical_centered_justified(|ui| {
                ui.add_visible_ui(self.is_option_menu_state(), |ui| {
                    if ui.button("Start timer").clicked() {
                        self.logger = Logger::new();
                        self.start();
                        ctx.request_repaint();
                    }
                });
                ui.add_visible_ui(self.is_counting_down_state(), |ui| {
                    if ui.button("Stop timer").clicked() {
                        self.stop();
                        ctx.request_repaint();
                        conditional_write(&mut self.logger);
                    }
                })
            });
        });
    }
}
