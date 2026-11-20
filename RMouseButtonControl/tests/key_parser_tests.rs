use rdev::Key;
use rmousebuttoncontrol::models::MouseButton;
use rmousebuttoncontrol::parser::{parse_keys, KeyOrButton};

#[test]
fn test_parse_simple_chord() {
    let seq = parse_keys("{ctrl}c").unwrap();
    assert_eq!(seq.len(), 2);
    assert_eq!(seq.keys[0].target, KeyOrButton::Key(Key::ControlLeft));
    assert!(seq.keys[0].is_modifier);
    assert_eq!(seq.keys[1].target, KeyOrButton::Key(Key::KeyC));
    assert!(!seq.keys[1].is_modifier);
}

#[test]
fn test_parse_multiple_modifiers() {
    let seq = parse_keys("{shift}{alt}t").unwrap();
    assert_eq!(seq.len(), 3);
    assert_eq!(seq.keys[0].target, KeyOrButton::Key(Key::ShiftLeft));
    assert!(seq.keys[0].is_modifier);
    assert_eq!(seq.keys[1].target, KeyOrButton::Key(Key::Alt));
    assert!(seq.keys[1].is_modifier);
    assert_eq!(seq.keys[2].target, KeyOrButton::Key(Key::KeyT));
    assert!(!seq.keys[2].is_modifier);
}

#[test]
fn test_parse_mouse_button_in_sequence() {
    let seq = parse_keys("{mb1}{ctrl}").unwrap();
    assert_eq!(seq.len(), 2);
    assert_eq!(
        seq.keys[0].target,
        KeyOrButton::Mouse(MouseButton::MouseButton1)
    );
    assert!(!seq.keys[0].is_modifier);
    assert_eq!(seq.keys[1].target, KeyOrButton::Key(Key::ControlLeft));
    assert!(seq.keys[1].is_modifier);
}

#[test]
fn test_parse_case_insensitivity() {
    let seq = parse_keys("{SHIFT}AbC").unwrap();
    assert_eq!(seq.len(), 4);
    assert_eq!(seq.keys[0].target, KeyOrButton::Key(Key::ShiftLeft));
    assert_eq!(seq.keys[1].target, KeyOrButton::Key(Key::KeyA));
    assert_eq!(seq.keys[2].target, KeyOrButton::Key(Key::KeyB));
    assert_eq!(seq.keys[3].target, KeyOrButton::Key(Key::KeyC));
}

#[test]
fn test_parse_special_keys() {
    let seq = parse_keys("{esc}{f1}{return}{space}{backspace}").unwrap();
    assert_eq!(seq.len(), 5);
    assert_eq!(seq.keys[0].target, KeyOrButton::Key(Key::Escape));
    assert_eq!(seq.keys[1].target, KeyOrButton::Key(Key::F1));
    assert_eq!(seq.keys[2].target, KeyOrButton::Key(Key::Return));
    assert_eq!(seq.keys[3].target, KeyOrButton::Key(Key::Space));
    assert_eq!(seq.keys[4].target, KeyOrButton::Key(Key::Backspace));
}

#[test]
fn test_parse_errors() {
    assert!(parse_keys("{unknown_key}").is_err());
    assert!(parse_keys("{unclosed").is_err());
}
