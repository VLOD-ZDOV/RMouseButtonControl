use super::{mapping::ButtonMapping, mouse_button::MouseButton};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub process: String,
    pub window_caption: String,
    pub window_class: String,
    pub parent_class: String,
    pub match_type: String,
    pub checked: bool,

    pub mouse_button_1: ButtonMapping,
    pub mouse_button_2: ButtonMapping,
    pub mouse_button_3: ButtonMapping,
    pub mouse_button_4: ButtonMapping,
    pub mouse_button_5: ButtonMapping,
    pub mouse_wheel_up: ButtonMapping,
    pub mouse_wheel_down: ButtonMapping,
    pub mouse_wheel_left: ButtonMapping,
    pub mouse_wheel_right: ButtonMapping,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            description: "Default profile".to_string(),
            process: "*".to_string(),
            window_caption: "*".to_string(),
            window_class: "*".to_string(),
            parent_class: "*".to_string(),
            match_type: "N/A".to_string(),
            checked: true,
            mouse_button_1: ButtonMapping::nothing(),
            mouse_button_2: ButtonMapping::nothing(),
            mouse_button_3: ButtonMapping::nothing(),
            mouse_button_4: ButtonMapping::nothing(),
            mouse_button_5: ButtonMapping::nothing(),
            mouse_wheel_up: ButtonMapping::nothing(),
            mouse_wheel_down: ButtonMapping::nothing(),
            mouse_wheel_left: ButtonMapping::nothing(),
            mouse_wheel_right: ButtonMapping::nothing(),
        }
    }
}

impl Profile {
    #[must_use]
    pub fn new(name: impl Into<String>, process: impl Into<String>, checked: bool) -> Self {
        Self {
            name: name.into(),
            process: process.into(),
            checked,
            ..Self::default()
        }
    }

    #[must_use]
    pub fn get_mapping(&self, button: MouseButton) -> &ButtonMapping {
        match button {
            MouseButton::MouseButton1 => &self.mouse_button_1,
            MouseButton::MouseButton2 => &self.mouse_button_2,
            MouseButton::MouseButton3 => &self.mouse_button_3,
            MouseButton::MouseButton4 => &self.mouse_button_4,
            MouseButton::MouseButton5 => &self.mouse_button_5,
            MouseButton::MouseWheelUp => &self.mouse_wheel_up,
            MouseButton::MouseWheelDown => &self.mouse_wheel_down,
            MouseButton::MouseWheelLeft => &self.mouse_wheel_left,
            MouseButton::MouseWheelRight => &self.mouse_wheel_right,
        }
    }

    pub fn set_mapping(&mut self, button: MouseButton, mapping: ButtonMapping) {
        match button {
            MouseButton::MouseButton1 => self.mouse_button_1 = mapping,
            MouseButton::MouseButton2 => self.mouse_button_2 = mapping,
            MouseButton::MouseButton3 => self.mouse_button_3 = mapping,
            MouseButton::MouseButton4 => self.mouse_button_4 = mapping,
            MouseButton::MouseButton5 => self.mouse_button_5 = mapping,
            MouseButton::MouseWheelUp => self.mouse_wheel_up = mapping,
            MouseButton::MouseWheelDown => self.mouse_wheel_down = mapping,
            MouseButton::MouseWheelLeft => self.mouse_wheel_left = mapping,
            MouseButton::MouseWheelRight => self.mouse_wheel_right = mapping,
        }
    }
}
