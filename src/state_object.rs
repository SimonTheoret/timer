use crate::guistate::GuiState;

#[derive(Clone)]
pub struct StateObject {
    pub duration: f32,
    pub gui_state: GuiState
}
