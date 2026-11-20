use super::Simulator;
use crate::models::MouseButton;
use rdev::{simulate, Button, EventType, Key};

pub struct UserspaceSimulator;

impl UserspaceSimulator {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    fn to_rdev_button(btn: MouseButton) -> Option<Button> {
        match btn {
            MouseButton::MouseButton1 => Some(Button::Left),
            MouseButton::MouseButton2 => Some(Button::Right),
            MouseButton::MouseButton3 => Some(Button::Middle),
            MouseButton::MouseButton4 => Some(Button::Unknown(1)),
            MouseButton::MouseButton5 => Some(Button::Unknown(2)),
            _ => None,
        }
    }
}

impl Default for UserspaceSimulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulator for UserspaceSimulator {
    fn press_key(&self, key: Key) {
        let _ = simulate(&EventType::KeyPress(key));
    }

    fn release_key(&self, key: Key) {
        let _ = simulate(&EventType::KeyRelease(key));
    }

    fn press_mouse(&self, button: MouseButton) {
        if let Some(btn) = Self::to_rdev_button(button) {
            let _ = simulate(&EventType::ButtonPress(btn));
        } else {
            match button {
                MouseButton::MouseWheelUp => {
                    let _ = simulate(&EventType::Wheel { delta_x: 0, delta_y: 1 });
                }
                MouseButton::MouseWheelDown => {
                    let _ = simulate(&EventType::Wheel { delta_x: 0, delta_y: -1 });
                }
                MouseButton::MouseWheelLeft => {
                    let _ = simulate(&EventType::Wheel { delta_x: -1, delta_y: 0 });
                }
                MouseButton::MouseWheelRight => {
                    let _ = simulate(&EventType::Wheel { delta_x: 1, delta_y: 0 });
                }
                _ => {}
            }
        }
    }

    fn release_mouse(&self, button: MouseButton) {
        if let Some(btn) = Self::to_rdev_button(button) {
            let _ = simulate(&EventType::ButtonRelease(btn));
        }
    }
}
