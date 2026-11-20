use super::Simulator;
use crate::models::MouseButton;
use rdev::Key;
use std::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulatedAction {
    PressKey(Key),
    ReleaseKey(Key),
    PressMouse(MouseButton),
    ReleaseMouse(MouseButton),
}

#[derive(Default)]
pub struct MockSimulator {
    pub actions: RwLock<Vec<SimulatedAction>>,
}

impl MockSimulator {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn recorded_actions(&self) -> Vec<SimulatedAction> {
        self.actions.read().unwrap().clone()
    }

    pub fn clear(&self) {
        self.actions.write().unwrap().clear();
    }
}

impl Simulator for MockSimulator {
    fn press_key(&self, key: Key) {
        self.actions
            .write()
            .unwrap()
            .push(SimulatedAction::PressKey(key));
    }

    fn release_key(&self, key: Key) {
        self.actions
            .write()
            .unwrap()
            .push(SimulatedAction::ReleaseKey(key));
    }

    fn press_mouse(&self, button: MouseButton) {
        self.actions
            .write()
            .unwrap()
            .push(SimulatedAction::PressMouse(button));
    }

    fn release_mouse(&self, button: MouseButton) {
        self.actions
            .write()
            .unwrap()
            .push(SimulatedAction::ReleaseMouse(button));
    }
}
