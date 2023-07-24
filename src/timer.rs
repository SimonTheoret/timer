use crate::guistate::GuiState;
use crate::state_object::StateObject;
use notify_rust::Notification;
use soloud::*;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct Timer {
    state: StateObject,
    rx_counting_down: Receiver<bool>,
    tx_state: Sender<StateObject>,
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
        }
    }
    fn send_state(&self) {
        let sending_state = self.state.clone();
        self.tx_state.send(sending_state).unwrap(); // Panic if the channel is down
    }
    fn handle_message(&self) -> bool {
        self.rx_counting_down.try_recv().unwrap_or_else(|_err| true)
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
        self.state.duration = 0.;
        self.send_state();
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
}
