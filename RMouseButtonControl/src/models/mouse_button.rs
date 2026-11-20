use serde::{Deserialize, Serialize};

/// 1:1 mapping of YMouseButton / RMouseButton
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum MouseButton {
    MouseButton1 = 1, // Left
    MouseButton2 = 2, // Right
    MouseButton3 = 3, // Middle
    MouseButton4 = 4, // XButton1 (Back)
    MouseButton5 = 5, // XButton2 (Forward)
    MouseWheelUp = 6,
    MouseWheelDown = 7,
    MouseWheelLeft = 8,
    MouseWheelRight = 9,
}

impl MouseButton {
    #[must_use]
    pub const fn is_wheel(self) -> bool {
        matches!(
            self,
            Self::MouseWheelUp
                | Self::MouseWheelDown
                | Self::MouseWheelLeft
                | Self::MouseWheelRight
        )
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::MouseButton1 => "MouseButton1",
            Self::MouseButton2 => "MouseButton2",
            Self::MouseButton3 => "MouseButton3",
            Self::MouseButton4 => "MouseButton4",
            Self::MouseButton5 => "MouseButton5",
            Self::MouseWheelUp => "MouseWheelUp",
            Self::MouseWheelDown => "MouseWheelDown",
            Self::MouseWheelLeft => "MouseWheelLeft",
            Self::MouseWheelRight => "MouseWheelRight",
        }
    }
}
