pub mod mock;
pub mod repeater;
pub mod sticky;
pub mod uinput_backend;
pub mod userspace_backend;

pub use mock::MockSimulator;
pub use repeater::RepeatWorker;
pub use sticky::{StickyHoldManager, StickyRepeatManager};
pub use uinput_backend::UInputSimulator;
pub use userspace_backend::UserspaceSimulator;

use crate::models::MouseButton;
use crate::parser::{KeyOrButton, ParsedKey, ParsedKeySequence};
use rdev::Key;
use std::thread;
use std::time::Duration;

pub trait Simulator: Send + Sync {
    fn press_key(&self, key: Key);
    fn release_key(&self, key: Key);
    fn press_mouse(&self, button: MouseButton);
    fn release_mouse(&self, button: MouseButton);

    fn press_target(&self, target: KeyOrButton) {
        match target {
            KeyOrButton::Key(k) => self.press_key(k),
            KeyOrButton::Mouse(m) => self.press_mouse(m),
        }
    }

    fn release_target(&self, target: KeyOrButton) {
        match target {
            KeyOrButton::Key(k) => self.release_key(k),
            KeyOrButton::Mouse(m) => self.release_mouse(m),
        }
    }

    /// Press all keys in forward order (1:1 with C# EventSimulatorService.PressKeys)
    fn press_sequence(&self, seq: &ParsedKeySequence) {
        for pk in &seq.keys {
            self.press_target(pk.target);
        }
    }

    /// Release all keys in reverse order (1:1 with C# EventSimulatorService.ReleaseKeys)
    fn release_sequence(&self, seq: &ParsedKeySequence) {
        for pk in seq.keys.iter().rev() {
            self.release_target(pk.target);
        }
    }

    /// Chord tapping with modifier tracking (1:1 with C# EventSimulatorService.TapKeys):
    /// Modifiers are held until the next non-modifier key is pressed,
    /// then the stack is popped and released in reverse order.
    fn tap_sequence(&self, seq: &ParsedKeySequence, delay_ms: Option<u32>) {
        let mut stack: Vec<&ParsedKey> = Vec::with_capacity(seq.keys.len());

        for pk in &seq.keys {
            // Pop the entire stack if the last key pressed is a normal key
            if let Some(peek) = stack.last() {
                if !peek.is_modifier {
                    while let Some(popped) = stack.pop() {
                        self.release_target(popped.target);
                    }
                }
            }

            stack.push(pk);
            self.press_target(pk.target);

            if let Some(delay) = delay_ms {
                if delay > 0 {
                    thread::sleep(Duration::from_millis(delay as u64));
                }
            }
        }

        // Release any remaining keys on the stack in reverse order
        while let Some(popped) = stack.pop() {
            self.release_target(popped.target);
        }
    }
}
