use crate::guistate::GuiState;
use crate::logger::{conditional_write, Logger};
use crate::state_object::StateObject;
use notify_rust::{Notification, Timeout};
use soloud::*;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct Timer {
    state: StateObject,
    rx_counting_down: Receiver<bool>,
    tx_state: Sender<StateObject>,
    logger: Option<Logger>,
}
impl Timer {
    pub fn new(
        state: StateObject,
        rx_counting_down: Receiver<bool>,
        tx_state: Sender<StateObject>,
    ) -> Timer {
        Timer {
            state,
            rx_counting_down,
            tx_state,
            logger: Logger::new(),
        }
    }
    fn send_state(&self) {
        let sending_state = self.state.clone();
        self.tx_state.send(sending_state).unwrap(); // Panic if the channel is down
    }
    fn handle_message(&self) -> bool {
        self.rx_counting_down.try_recv().unwrap_or_else(|_err| true)
    }
    fn kill_logger(&mut self) {
        self.logger = None
    }
    pub fn count_down(&mut self) {
        let mut counting_down = self.handle_message();
        let duration = std::time::Duration::new(1, 0);
        self.state.gui_state = GuiState::CountingDown;
        while self.state.duration >= 1. / 60. && counting_down {
            self.state.duration -= 1. / 60.;
            self.send_state();
            counting_down = self.handle_message();
            if counting_down {
                thread::sleep(duration);
            }
        }
        self.state.gui_state = GuiState::OptionMenu;
        if self.state.duration <= 1. / 60.{
            conditional_write(&mut self.logger);
            self.kill_logger();
        }
        self.state.duration = 0.;
        self.send_state();
        match Notification::new()
            .summary("Rust timer:")
            .body("Working period is done! \nTime to take a break")
            .appname("Rust timer")
            .timeout(Timeout::Never)
            .show()
        {
            Ok(_) => (),
            Err(_) => (),
        }
        let mut sl = Soloud::default().unwrap();
        let mut wav = audio::Wav::default();
        sl.set_global_volume(3.0);
        wav.load_mem(include_bytes!(
            "./mixkit-interface-hint-notification-911.wav"
        ))
        .unwrap();
        sl.play(&wav);
        while sl.voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}
