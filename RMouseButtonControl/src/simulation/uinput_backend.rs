use super::Simulator;
use crate::models::MouseButton;
use rdev::Key;
use std::io;

#[cfg(target_os = "linux")]
use evdev::uinput::VirtualDevice;
#[cfg(target_os = "linux")]
use evdev::{AttributeSet, EventType, InputEvent, KeyCode, SynchronizationCode};
#[cfg(target_os = "linux")]
use std::sync::Mutex;

/// Linux Kernel virtual input device backend (/dev/uinput)
///
/// Advantages over standard X11 / user-space injection:
/// - Works under native Wayland compositors (Hyprland, Sway, GNOME Wayland, KDE Wayland)
/// - Indistinguishable from real physical hardware (passes kernel input subsystem)
/// - Undetected by anti-cheat / Wine/Proton games that block synthetic X11/Win32 events
pub struct UInputSimulator {
    #[cfg(target_os = "linux")]
    device: Mutex<VirtualDevice>,
}

#[cfg(target_os = "linux")]
impl UInputSimulator {
    pub fn try_new() -> io::Result<Self> {
        let mut keys = AttributeSet::<KeyCode>::new();

        // Mouse buttons
        keys.insert(KeyCode::BTN_LEFT);
        keys.insert(KeyCode::BTN_RIGHT);
        keys.insert(KeyCode::BTN_MIDDLE);
        keys.insert(KeyCode::BTN_SIDE);
        keys.insert(KeyCode::BTN_EXTRA);

        // Core keyboard keys
        keys.insert(KeyCode::KEY_ESC);
        keys.insert(KeyCode::KEY_1);
        keys.insert(KeyCode::KEY_2);
        keys.insert(KeyCode::KEY_3);
        keys.insert(KeyCode::KEY_4);
        keys.insert(KeyCode::KEY_5);
        keys.insert(KeyCode::KEY_6);
        keys.insert(KeyCode::KEY_7);
        keys.insert(KeyCode::KEY_8);
        keys.insert(KeyCode::KEY_9);
        keys.insert(KeyCode::KEY_0);
        keys.insert(KeyCode::KEY_MINUS);
        keys.insert(KeyCode::KEY_EQUAL);
        keys.insert(KeyCode::KEY_BACKSPACE);
        keys.insert(KeyCode::KEY_TAB);
        keys.insert(KeyCode::KEY_Q);
        keys.insert(KeyCode::KEY_W);
        keys.insert(KeyCode::KEY_E);
        keys.insert(KeyCode::KEY_R);
        keys.insert(KeyCode::KEY_T);
        keys.insert(KeyCode::KEY_Y);
        keys.insert(KeyCode::KEY_U);
        keys.insert(KeyCode::KEY_I);
        keys.insert(KeyCode::KEY_O);
        keys.insert(KeyCode::KEY_P);
        keys.insert(KeyCode::KEY_LEFTBRACE);
        keys.insert(KeyCode::KEY_RIGHTBRACE);
        keys.insert(KeyCode::KEY_ENTER);
        keys.insert(KeyCode::KEY_LEFTCTRL);
        keys.insert(KeyCode::KEY_A);
        keys.insert(KeyCode::KEY_S);
        keys.insert(KeyCode::KEY_D);
        keys.insert(KeyCode::KEY_F);
        keys.insert(KeyCode::KEY_G);
        keys.insert(KeyCode::KEY_H);
        keys.insert(KeyCode::KEY_J);
        keys.insert(KeyCode::KEY_K);
        keys.insert(KeyCode::KEY_L);
        keys.insert(KeyCode::KEY_SEMICOLON);
        keys.insert(KeyCode::KEY_APOSTROPHE);
        keys.insert(KeyCode::KEY_LEFTSHIFT);
        keys.insert(KeyCode::KEY_Z);
        keys.insert(KeyCode::KEY_X);
        keys.insert(KeyCode::KEY_C);
        keys.insert(KeyCode::KEY_V);
        keys.insert(KeyCode::KEY_B);
        keys.insert(KeyCode::KEY_N);
        keys.insert(KeyCode::KEY_M);
        keys.insert(KeyCode::KEY_COMMA);
        keys.insert(KeyCode::KEY_DOT);
        keys.insert(KeyCode::KEY_SLASH);
        keys.insert(KeyCode::KEY_RIGHTSHIFT);
        keys.insert(KeyCode::KEY_LEFTALT);
        keys.insert(KeyCode::KEY_SPACE);
        keys.insert(KeyCode::KEY_CAPSLOCK);
        keys.insert(KeyCode::KEY_F1);
        keys.insert(KeyCode::KEY_F2);
        keys.insert(KeyCode::KEY_F3);
        keys.insert(KeyCode::KEY_F4);
        keys.insert(KeyCode::KEY_F5);
        keys.insert(KeyCode::KEY_F6);
        keys.insert(KeyCode::KEY_F7);
        keys.insert(KeyCode::KEY_F8);
        keys.insert(KeyCode::KEY_F9);
        keys.insert(KeyCode::KEY_F10);
        keys.insert(KeyCode::KEY_F11);
        keys.insert(KeyCode::KEY_F12);
        keys.insert(KeyCode::KEY_RIGHTCTRL);
        keys.insert(KeyCode::KEY_RIGHTALT);
        keys.insert(KeyCode::KEY_LEFTMETA);
        keys.insert(KeyCode::KEY_RIGHTMETA);
        keys.insert(KeyCode::KEY_UP);
        keys.insert(KeyCode::KEY_LEFT);
        keys.insert(KeyCode::KEY_RIGHT);
        keys.insert(KeyCode::KEY_DOWN);
        keys.insert(KeyCode::KEY_DELETE);
        keys.insert(KeyCode::KEY_INSERT);
        keys.insert(KeyCode::KEY_HOME);
        keys.insert(KeyCode::KEY_END);
        keys.insert(KeyCode::KEY_PAGEUP);
        keys.insert(KeyCode::KEY_PAGEDOWN);

        let device = VirtualDevice::builder()?
            .name("RMouseButtonControl Kernel Device")
            .with_keys(&keys)?
            .build()?;

        Ok(Self {
            device: Mutex::new(device),
        })
    }

    fn to_evdev_key(key: Key) -> Option<KeyCode> {
        match key {
            Key::KeyA => Some(KeyCode::KEY_A),
            Key::KeyB => Some(KeyCode::KEY_B),
            Key::KeyC => Some(KeyCode::KEY_C),
            Key::KeyD => Some(KeyCode::KEY_D),
            Key::KeyE => Some(KeyCode::KEY_E),
            Key::KeyF => Some(KeyCode::KEY_F),
            Key::KeyG => Some(KeyCode::KEY_G),
            Key::KeyH => Some(KeyCode::KEY_H),
            Key::KeyI => Some(KeyCode::KEY_I),
            Key::KeyJ => Some(KeyCode::KEY_J),
            Key::KeyK => Some(KeyCode::KEY_K),
            Key::KeyL => Some(KeyCode::KEY_L),
            Key::KeyM => Some(KeyCode::KEY_M),
            Key::KeyN => Some(KeyCode::KEY_N),
            Key::KeyO => Some(KeyCode::KEY_O),
            Key::KeyP => Some(KeyCode::KEY_P),
            Key::KeyQ => Some(KeyCode::KEY_Q),
            Key::KeyR => Some(KeyCode::KEY_R),
            Key::KeyS => Some(KeyCode::KEY_S),
            Key::KeyT => Some(KeyCode::KEY_T),
            Key::KeyU => Some(KeyCode::KEY_U),
            Key::KeyV => Some(KeyCode::KEY_V),
            Key::KeyW => Some(KeyCode::KEY_W),
            Key::KeyX => Some(KeyCode::KEY_X),
            Key::KeyY => Some(KeyCode::KEY_Y),
            Key::KeyZ => Some(KeyCode::KEY_Z),
            Key::Num1 => Some(KeyCode::KEY_1),
            Key::Num2 => Some(KeyCode::KEY_2),
            Key::Num3 => Some(KeyCode::KEY_3),
            Key::Num4 => Some(KeyCode::KEY_4),
            Key::Num5 => Some(KeyCode::KEY_5),
            Key::Num6 => Some(KeyCode::KEY_6),
            Key::Num7 => Some(KeyCode::KEY_7),
            Key::Num8 => Some(KeyCode::KEY_8),
            Key::Num9 => Some(KeyCode::KEY_9),
            Key::Num0 => Some(KeyCode::KEY_0),
            Key::Escape => Some(KeyCode::KEY_ESC),
            Key::Return => Some(KeyCode::KEY_ENTER),
            Key::Space => Some(KeyCode::KEY_SPACE),
            Key::Tab => Some(KeyCode::KEY_TAB),
            Key::Backspace => Some(KeyCode::KEY_BACKSPACE),
            Key::ShiftLeft => Some(KeyCode::KEY_LEFTSHIFT),
            Key::ShiftRight => Some(KeyCode::KEY_RIGHTSHIFT),
            Key::ControlLeft => Some(KeyCode::KEY_LEFTCTRL),
            Key::ControlRight => Some(KeyCode::KEY_RIGHTCTRL),
            Key::Alt => Some(KeyCode::KEY_LEFTALT),
            Key::AltGr => Some(KeyCode::KEY_RIGHTALT),
            Key::MetaLeft => Some(KeyCode::KEY_LEFTMETA),
            Key::MetaRight => Some(KeyCode::KEY_RIGHTMETA),
            Key::F1 => Some(KeyCode::KEY_F1),
            Key::F2 => Some(KeyCode::KEY_F2),
            Key::F3 => Some(KeyCode::KEY_F3),
            Key::F4 => Some(KeyCode::KEY_F4),
            Key::F5 => Some(KeyCode::KEY_F5),
            Key::F6 => Some(KeyCode::KEY_F6),
            Key::F7 => Some(KeyCode::KEY_F7),
            Key::F8 => Some(KeyCode::KEY_F8),
            Key::F9 => Some(KeyCode::KEY_F9),
            Key::F10 => Some(KeyCode::KEY_F10),
            Key::F11 => Some(KeyCode::KEY_F11),
            Key::F12 => Some(KeyCode::KEY_F12),
            Key::UpArrow => Some(KeyCode::KEY_UP),
            Key::DownArrow => Some(KeyCode::KEY_DOWN),
            Key::LeftArrow => Some(KeyCode::KEY_LEFT),
            Key::RightArrow => Some(KeyCode::KEY_RIGHT),
            Key::Delete => Some(KeyCode::KEY_DELETE),
            Key::Insert => Some(KeyCode::KEY_INSERT),
            Key::Home => Some(KeyCode::KEY_HOME),
            Key::End => Some(KeyCode::KEY_END),
            Key::PageUp => Some(KeyCode::KEY_PAGEUP),
            Key::PageDown => Some(KeyCode::KEY_PAGEDOWN),
            _ => None,
        }
    }

    fn to_evdev_mouse_button(btn: MouseButton) -> Option<KeyCode> {
        match btn {
            MouseButton::MouseButton1 => Some(KeyCode::BTN_LEFT),
            MouseButton::MouseButton2 => Some(KeyCode::BTN_RIGHT),
            MouseButton::MouseButton3 => Some(KeyCode::BTN_MIDDLE),
            MouseButton::MouseButton4 => Some(KeyCode::BTN_SIDE),
            MouseButton::MouseButton5 => Some(KeyCode::BTN_EXTRA),
            _ => None,
        }
    }

    fn emit_key_code(&self, code: KeyCode, down: bool) {
        let val = if down { 1 } else { 0 };
        if let Ok(mut dev) = self.device.lock() {
            let _ = dev.emit(&[
                InputEvent::new(EventType::KEY.0, code.code(), val),
                InputEvent::new(
                    EventType::SYNCHRONIZATION.0,
                    SynchronizationCode::SYN_REPORT.0,
                    0,
                ),
            ]);
        }
    }
}

#[cfg(not(target_os = "linux"))]
impl UInputSimulator {
    pub fn try_new() -> io::Result<Self> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "/dev/uinput is only supported on Linux",
        ))
    }
}

impl Simulator for UInputSimulator {
    fn press_key(&self, key: Key) {
        #[cfg(target_os = "linux")]
        if let Some(code) = Self::to_evdev_key(key) {
            self.emit_key_code(code, true);
        }
        #[cfg(not(target_os = "linux"))]
        let _ = key;
    }

    fn release_key(&self, key: Key) {
        #[cfg(target_os = "linux")]
        if let Some(code) = Self::to_evdev_key(key) {
            self.emit_key_code(code, false);
        }
        #[cfg(not(target_os = "linux"))]
        let _ = key;
    }

    fn press_mouse(&self, button: MouseButton) {
        #[cfg(target_os = "linux")]
        if let Some(code) = Self::to_evdev_mouse_button(button) {
            self.emit_key_code(code, true);
        }
        #[cfg(not(target_os = "linux"))]
        let _ = button;
    }

    fn release_mouse(&self, button: MouseButton) {
        #[cfg(target_os = "linux")]
        if let Some(code) = Self::to_evdev_mouse_button(button) {
            self.emit_key_code(code, false);
        }
        #[cfg(not(target_os = "linux"))]
        let _ = button;
    }
}
