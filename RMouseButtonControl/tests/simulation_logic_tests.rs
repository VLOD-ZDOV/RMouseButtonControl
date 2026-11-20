use rdev::Key;
use rmousebuttoncontrol::engine::{EventDispatcher, ProfilesCache};
use rmousebuttoncontrol::models::{ActionType, ButtonMapping, MouseButton, MouseButtonState, Profile};
use rmousebuttoncontrol::parser::parse_keys;
use rmousebuttoncontrol::simulation::mock::SimulatedAction;
use rmousebuttoncontrol::simulation::{MockSimulator, RepeatWorker, Simulator};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[test]
fn test_chord_tap_release_in_reverse_order() {
    let mock = MockSimulator::new();
    let seq = parse_keys("{ctrl}c").unwrap();

    mock.tap_sequence(&seq, None);

    let actions = mock.recorded_actions();
    assert_eq!(
        actions,
        vec![
            SimulatedAction::PressKey(Key::ControlLeft),
            SimulatedAction::PressKey(Key::KeyC),
            SimulatedAction::ReleaseKey(Key::KeyC),
            SimulatedAction::ReleaseKey(Key::ControlLeft),
        ]
    );
}

#[test]
fn test_during_mouse_press_and_release() {
    let mock = Arc::new(MockSimulator::new());
    let cache = Arc::new(ProfilesCache::new());

    let mut profile = Profile::new("TestProfile", "*", true);
    profile.set_mapping(
        MouseButton::MouseButton4,
        ButtonMapping::simulated_keystroke(
            ActionType::DuringMousePressAndRelease,
            "{shift}x",
            true,
            None,
            None,
        ),
    );
    cache.set_profiles(vec![profile]);

    let dispatcher = EventDispatcher::new(cache, Arc::clone(&mock), Arc::new(AtomicBool::new(true)));

    // Mouse button pressed
    let disp = dispatcher.process_event(
        MouseButton::MouseButton4,
        MouseButtonState::Pressed,
        "/usr/bin/app",
    );
    assert!(disp.should_suppress);
    assert_eq!(disp.matched_profiles, 1);

    assert_eq!(
        mock.recorded_actions(),
        vec![
            SimulatedAction::PressKey(Key::ShiftLeft),
            SimulatedAction::PressKey(Key::KeyX),
        ]
    );

    mock.clear();

    // Mouse button released -> releases in reverse order
    dispatcher.process_event(
        MouseButton::MouseButton4,
        MouseButtonState::Released,
        "/usr/bin/app",
    );
    assert_eq!(
        mock.recorded_actions(),
        vec![
            SimulatedAction::ReleaseKey(Key::KeyX),
            SimulatedAction::ReleaseKey(Key::ShiftLeft),
        ]
    );
}

#[test]
fn test_right_click_simulation() {
    let mock = Arc::new(MockSimulator::new());
    let cache = Arc::new(ProfilesCache::new());

    let mut profile = Profile::new("TestProfile", "*", true);
    profile.set_mapping(
        MouseButton::MouseButton4,
        ButtonMapping::right_click(true),
    );
    cache.set_profiles(vec![profile]);

    let dispatcher = EventDispatcher::new(cache, Arc::clone(&mock), Arc::new(AtomicBool::new(true)));

    dispatcher.process_event(
        MouseButton::MouseButton4,
        MouseButtonState::Pressed,
        "*",
    );
    assert_eq!(
        mock.recorded_actions(),
        vec![SimulatedAction::PressMouse(MouseButton::MouseButton2)]
    );

    mock.clear();

    dispatcher.process_event(
        MouseButton::MouseButton4,
        MouseButtonState::Released,
        "*",
    );
    assert_eq!(
        mock.recorded_actions(),
        vec![SimulatedAction::ReleaseMouse(MouseButton::MouseButton2)]
    );
}

#[test]
fn test_sticky_hold_toggle() {
    let mock = Arc::new(MockSimulator::new());
    let cache = Arc::new(ProfilesCache::new());

    let mut profile = Profile::new("TestProfile", "*", true);
    profile.set_mapping(
        MouseButton::MouseButton4,
        ButtonMapping::simulated_keystroke(
            ActionType::StickyHold,
            "{ctrl}",
            false,
            None,
            None,
        ),
    );
    cache.set_profiles(vec![profile]);

    let dispatcher = EventDispatcher::new(cache, Arc::clone(&mock), Arc::new(AtomicBool::new(true)));

    // 1st press -> holds key
    dispatcher.process_event(
        MouseButton::MouseButton4,
        MouseButtonState::Pressed,
        "*",
    );
    assert_eq!(
        mock.recorded_actions(),
        vec![SimulatedAction::PressKey(Key::ControlLeft)]
    );

    mock.clear();

    // 2nd press -> releases key
    dispatcher.process_event(
        MouseButton::MouseButton4,
        MouseButtonState::Pressed,
        "*",
    );
    assert_eq!(
        mock.recorded_actions(),
        vec![SimulatedAction::ReleaseKey(Key::ControlLeft)]
    );
}

#[test]
fn test_suppression_check() {
    let mock = Arc::new(MockSimulator::new());
    let cache = Arc::new(ProfilesCache::new());

    let mut profile = Profile::new("FirefoxProfile", "firefox", true);
    profile.set_mapping(
        MouseButton::MouseButton4,
        ButtonMapping::disabled(true), // block original MB4 in firefox
    );
    cache.set_profiles(vec![profile]);

    let dispatcher = EventDispatcher::new(cache, mock, Arc::new(AtomicBool::new(true)));

    assert!(dispatcher.should_suppress_event(MouseButton::MouseButton4, "/usr/bin/firefox"));
    assert!(!dispatcher.should_suppress_event(MouseButton::MouseButton4, "/usr/bin/chrome"));
    assert!(!dispatcher.should_suppress_event(MouseButton::MouseButton1, "/usr/bin/firefox"));
}

#[test]
fn test_jitter_computation() {
    let base_delay = 100;
    for _ in 0..100 {
        let delay = RepeatWorker::compute_delay_ms(base_delay, true);
        // Delay with 10% jitter should be within [90, 110]
        assert!(delay >= 90 && delay <= 110, "Delay {delay} out of range");
    }

    assert_eq!(RepeatWorker::compute_delay_ms(base_delay, false), 100);
}
