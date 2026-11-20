use crate::models::MouseButton;
use rdev::Key;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyOrButton {
    Key(Key),
    Mouse(MouseButton),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedKey {
    pub target: KeyOrButton,
    pub is_modifier: bool,
    pub raw: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedKeySequence {
    pub keys: Vec<ParsedKey>,
}

impl ParsedKeySequence {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.keys.len()
    }
}

static KEY_MAP: OnceLock<HashMap<&'static str, (KeyOrButton, bool)>> = OnceLock::new();

fn get_key_map() -> &'static HashMap<&'static str, (KeyOrButton, bool)> {
    KEY_MAP.get_or_init(|| {
        let mut m = HashMap::new();

        // Modifiers (is_modifier = true)
        m.insert("shift", (KeyOrButton::Key(Key::ShiftLeft), true));
        m.insert("lshift", (KeyOrButton::Key(Key::ShiftLeft), true));
        m.insert("rshift", (KeyOrButton::Key(Key::ShiftRight), true));
        m.insert("ctrl", (KeyOrButton::Key(Key::ControlLeft), true));
        m.insert("lctrl", (KeyOrButton::Key(Key::ControlLeft), true));
        m.insert("rctrl", (KeyOrButton::Key(Key::ControlRight), true));
        m.insert("alt", (KeyOrButton::Key(Key::Alt), true));
        m.insert("lalt", (KeyOrButton::Key(Key::Alt), true));
        m.insert("ralt", (KeyOrButton::Key(Key::AltGr), true));
        m.insert("lwin", (KeyOrButton::Key(Key::MetaLeft), true));
        m.insert("rwin", (KeyOrButton::Key(Key::MetaRight), true));
        m.insert("win", (KeyOrButton::Key(Key::MetaLeft), true));

        // Function keys
        m.insert("f1", (KeyOrButton::Key(Key::F1), false));
        m.insert("f2", (KeyOrButton::Key(Key::F2), false));
        m.insert("f3", (KeyOrButton::Key(Key::F3), false));
        m.insert("f4", (KeyOrButton::Key(Key::F4), false));
        m.insert("f5", (KeyOrButton::Key(Key::F5), false));
        m.insert("f6", (KeyOrButton::Key(Key::F6), false));
        m.insert("f7", (KeyOrButton::Key(Key::F7), false));
        m.insert("f8", (KeyOrButton::Key(Key::F8), false));
        m.insert("f9", (KeyOrButton::Key(Key::F9), false));
        m.insert("f10", (KeyOrButton::Key(Key::F10), false));
        m.insert("f11", (KeyOrButton::Key(Key::F11), false));
        m.insert("f12", (KeyOrButton::Key(Key::F12), false));

        // Control / Navigation
        m.insert("esc", (KeyOrButton::Key(Key::Escape), false));
        m.insert("escape", (KeyOrButton::Key(Key::Escape), false));
        m.insert("tab", (KeyOrButton::Key(Key::Tab), false));
        m.insert("capslock", (KeyOrButton::Key(Key::CapsLock), false));
        m.insert("return", (KeyOrButton::Key(Key::Return), false));
        m.insert("enter", (KeyOrButton::Key(Key::Return), false));
        m.insert("backspace", (KeyOrButton::Key(Key::Backspace), false));
        m.insert("space", (KeyOrButton::Key(Key::Space), false));
        m.insert("prtscn", (KeyOrButton::Key(Key::PrintScreen), false));
        m.insert("printscreen", (KeyOrButton::Key(Key::PrintScreen), false));
        m.insert("scrolllock", (KeyOrButton::Key(Key::ScrollLock), false));
        m.insert("pause", (KeyOrButton::Key(Key::Pause), false));
        m.insert("ins", (KeyOrButton::Key(Key::Insert), false));
        m.insert("insert", (KeyOrButton::Key(Key::Insert), false));
        m.insert("del", (KeyOrButton::Key(Key::Delete), false));
        m.insert("delete", (KeyOrButton::Key(Key::Delete), false));
        m.insert("home", (KeyOrButton::Key(Key::Home), false));
        m.insert("end", (KeyOrButton::Key(Key::End), false));
        m.insert("pgup", (KeyOrButton::Key(Key::PageUp), false));
        m.insert("pageup", (KeyOrButton::Key(Key::PageUp), false));
        m.insert("pgdn", (KeyOrButton::Key(Key::PageDown), false));
        m.insert("pagedown", (KeyOrButton::Key(Key::PageDown), false));
        m.insert("up", (KeyOrButton::Key(Key::UpArrow), false));
        m.insert("down", (KeyOrButton::Key(Key::DownArrow), false));
        m.insert("left", (KeyOrButton::Key(Key::LeftArrow), false));
        m.insert("right", (KeyOrButton::Key(Key::RightArrow), false));

        // Numbers & punctuation
        m.insert("1", (KeyOrButton::Key(Key::Num1), false));
        m.insert("2", (KeyOrButton::Key(Key::Num2), false));
        m.insert("3", (KeyOrButton::Key(Key::Num3), false));
        m.insert("4", (KeyOrButton::Key(Key::Num4), false));
        m.insert("5", (KeyOrButton::Key(Key::Num5), false));
        m.insert("6", (KeyOrButton::Key(Key::Num6), false));
        m.insert("7", (KeyOrButton::Key(Key::Num7), false));
        m.insert("8", (KeyOrButton::Key(Key::Num8), false));
        m.insert("9", (KeyOrButton::Key(Key::Num9), false));
        m.insert("0", (KeyOrButton::Key(Key::Num0), false));
        m.insert("-", (KeyOrButton::Key(Key::Minus), false));
        m.insert("=", (KeyOrButton::Key(Key::Equal), false));
        m.insert("[", (KeyOrButton::Key(Key::LeftBracket), false));
        m.insert("]", (KeyOrButton::Key(Key::RightBracket), false));
        m.insert(";", (KeyOrButton::Key(Key::SemiColon), false));
        m.insert("'", (KeyOrButton::Key(Key::Quote), false));
        m.insert("\"", (KeyOrButton::Key(Key::Quote), false));
        m.insert(",", (KeyOrButton::Key(Key::Comma), false));
        m.insert(".", (KeyOrButton::Key(Key::Dot), false));
        m.insert("/", (KeyOrButton::Key(Key::Slash), false));
        m.insert(" ", (KeyOrButton::Key(Key::Space), false));

        // Numpad
        m.insert("numlock", (KeyOrButton::Key(Key::NumLock), false));
        m.insert("num/", (KeyOrButton::Key(Key::KpDivide), false));
        m.insert("num*", (KeyOrButton::Key(Key::KpMultiply), false));
        m.insert("num-", (KeyOrButton::Key(Key::KpMinus), false));
        m.insert("num+", (KeyOrButton::Key(Key::KpPlus), false));
        m.insert("numenter", (KeyOrButton::Key(Key::KpReturn), false));
        m.insert("num.", (KeyOrButton::Key(Key::KpDelete), false));
        m.insert("num0", (KeyOrButton::Key(Key::Kp0), false));
        m.insert("num1", (KeyOrButton::Key(Key::Kp1), false));
        m.insert("num2", (KeyOrButton::Key(Key::Kp2), false));
        m.insert("num3", (KeyOrButton::Key(Key::Kp3), false));
        m.insert("num4", (KeyOrButton::Key(Key::Kp4), false));
        m.insert("num5", (KeyOrButton::Key(Key::Kp5), false));
        m.insert("num6", (KeyOrButton::Key(Key::Kp6), false));
        m.insert("num7", (KeyOrButton::Key(Key::Kp7), false));
        m.insert("num8", (KeyOrButton::Key(Key::Kp8), false));
        m.insert("num9", (KeyOrButton::Key(Key::Kp9), false));

        // Letters a-z
        m.insert("a", (KeyOrButton::Key(Key::KeyA), false));
        m.insert("b", (KeyOrButton::Key(Key::KeyB), false));
        m.insert("c", (KeyOrButton::Key(Key::KeyC), false));
        m.insert("d", (KeyOrButton::Key(Key::KeyD), false));
        m.insert("e", (KeyOrButton::Key(Key::KeyE), false));
        m.insert("f", (KeyOrButton::Key(Key::KeyF), false));
        m.insert("g", (KeyOrButton::Key(Key::KeyG), false));
        m.insert("h", (KeyOrButton::Key(Key::KeyH), false));
        m.insert("i", (KeyOrButton::Key(Key::KeyI), false));
        m.insert("j", (KeyOrButton::Key(Key::KeyJ), false));
        m.insert("k", (KeyOrButton::Key(Key::KeyK), false));
        m.insert("l", (KeyOrButton::Key(Key::KeyL), false));
        m.insert("m", (KeyOrButton::Key(Key::KeyM), false));
        m.insert("n", (KeyOrButton::Key(Key::KeyN), false));
        m.insert("o", (KeyOrButton::Key(Key::KeyO), false));
        m.insert("p", (KeyOrButton::Key(Key::KeyP), false));
        m.insert("q", (KeyOrButton::Key(Key::KeyQ), false));
        m.insert("r", (KeyOrButton::Key(Key::KeyR), false));
        m.insert("s", (KeyOrButton::Key(Key::KeyS), false));
        m.insert("t", (KeyOrButton::Key(Key::KeyT), false));
        m.insert("u", (KeyOrButton::Key(Key::KeyU), false));
        m.insert("v", (KeyOrButton::Key(Key::KeyV), false));
        m.insert("w", (KeyOrButton::Key(Key::KeyW), false));
        m.insert("x", (KeyOrButton::Key(Key::KeyX), false));
        m.insert("y", (KeyOrButton::Key(Key::KeyY), false));
        m.insert("z", (KeyOrButton::Key(Key::KeyZ), false));

        // Mouse buttons in sequence
        m.insert("mb1", (KeyOrButton::Mouse(MouseButton::MouseButton1), false));
        m.insert("lmb", (KeyOrButton::Mouse(MouseButton::MouseButton1), false));
        m.insert("mb2", (KeyOrButton::Mouse(MouseButton::MouseButton2), false));
        m.insert("rmb", (KeyOrButton::Mouse(MouseButton::MouseButton2), false));
        m.insert("mb3", (KeyOrButton::Mouse(MouseButton::MouseButton3), false));
        m.insert("mmb", (KeyOrButton::Mouse(MouseButton::MouseButton3), false));
        m.insert("mb4", (KeyOrButton::Mouse(MouseButton::MouseButton4), false));
        m.insert("mb5", (KeyOrButton::Mouse(MouseButton::MouseButton5), false));

        m
    })
}

/// Parses key definitions like "{SHIFT}abc", "{CTRL}c", "{ESC}"
/// 1:1 parity with C# EventSimulatorService.ParseKeys
pub fn parse_keys(input: &str) -> Result<ParsedKeySequence, String> {
    let map = get_key_map();
    let chars: Vec<char> = input.chars().collect();
    let mut keys = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '{' {
            let mut found_end = None;
            for j in (i + 1)..chars.len() {
                if chars[j] == '}' {
                    found_end = Some(j);
                    break;
                }
            }

            if let Some(j) = found_end {
                let tag: String = chars[(i + 1)..j].iter().collect();
                let lower_tag = tag.to_lowercase();
                if let Some(&(target, is_mod)) = map.get(lower_tag.as_str()) {
                    keys.push(ParsedKey {
                        target,
                        is_modifier: is_mod,
                        raw: lower_tag,
                    });
                } else {
                    return Err(format!("Unknown key in tag: {{{tag}}}"));
                }
                i = j + 1;
            } else {
                return Err(format!("Unclosed '{{' at position {i}"));
            }
        } else {
            let c = chars[i].to_ascii_lowercase();
            let c_str = c.to_string();
            if let Some(&(target, is_mod)) = map.get(c_str.as_str()) {
                keys.push(ParsedKey {
                    target,
                    is_modifier: is_mod,
                    raw: c_str,
                });
            } else {
                return Err(format!("Unsupported character '{c}' at position {i}"));
            }
            i += 1;
        }
    }

    Ok(ParsedKeySequence { keys })
}
