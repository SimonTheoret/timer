use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::guistate::GuiState;
use crate::state_object::StateObject;

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
        self.send_state();
    }
}
