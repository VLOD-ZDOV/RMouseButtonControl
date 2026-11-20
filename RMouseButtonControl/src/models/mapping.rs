use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    /// Passthrough original behavior (no action)
    Nothing,
    /// Disable button (swallowed if block_original_mouse_input is true, no action performed)
    Disabled,
    /// Simulate standard Right Click
    RightClick,
    /// Mode 1: Tap key sequence when button is pressed
    AsMouseButtonPressed,
    /// Mode 2: Tap key sequence when button is released
    AsMouseButtonReleased,
    /// Mode 3: Hold key sequence while mouse button is held, release when mouse button released
    DuringMousePressAndRelease,
    /// Mode 4: Repeatedly tap keys while button is held down
    RepeatedWhileButtonDown,
    /// Mode 5: Toggle sticky repeat on/off with each press
    StickyRepeat,
    /// Mode 6: Toggle sticky key hold on/off with each press
    StickyHold,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ButtonMapping {
    pub action: ActionType,
    pub keys: Option<String>,
    pub auto_repeat_delay: Option<u32>,
    pub auto_repeat_randomize_delay_enabled: Option<bool>,
    pub block_original_mouse_input: bool,
}

impl Default for ButtonMapping {
    fn default() -> Self {
        Self {
            action: ActionType::Nothing,
            keys: None,
            auto_repeat_delay: Some(33),
            auto_repeat_randomize_delay_enabled: Some(false),
            block_original_mouse_input: false,
        }
    }
}

impl ButtonMapping {
    pub fn nothing() -> Self {
        Self::default()
    }

    pub fn disabled(block: bool) -> Self {
        Self {
            action: ActionType::Disabled,
            block_original_mouse_input: block,
            ..Self::default()
        }
    }

    pub fn right_click(block: bool) -> Self {
        Self {
            action: ActionType::RightClick,
            block_original_mouse_input: block,
            ..Self::default()
        }
    }

    pub fn simulated_keystroke(
        action: ActionType,
        keys: impl Into<String>,
        block: bool,
        delay: Option<u32>,
        randomize: Option<bool>,
    ) -> Self {
        Self {
            action,
            keys: Some(keys.into()),
            auto_repeat_delay: delay.or(Some(33)),
            auto_repeat_randomize_delay_enabled: randomize.or(Some(false)),
            block_original_mouse_input: block,
        }
    }
}
