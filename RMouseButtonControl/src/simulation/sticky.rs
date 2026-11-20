use super::repeater::RepeatWorker;
use crate::parser::ParsedKeySequence;
use std::collections::HashMap;

/// Manages toggleable state for StickyHold (Mode 6)
#[derive(Debug, Default)]
pub struct StickyHoldManager {
    /// Tracks which button/profile key is currently held down
    held_states: HashMap<String, bool>,
}

impl StickyHoldManager {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Toggles the held state for a given ID.
    /// Returns `true` if it transitioned to HELD (should press keys),
    /// and `false` if it transitioned to RELEASED (should release keys).
    pub fn toggle(&mut self, id: &str) -> bool {
        let entry = self.held_states.entry(id.to_string()).or_insert(false);
        *entry = !*entry;
        *entry
    }

    #[must_use]
    pub fn is_held(&self, id: &str) -> bool {
        self.held_states.get(id).copied().unwrap_or(false)
    }

    pub fn reset(&mut self, id: &str) {
        self.held_states.remove(id);
    }
}

/// Manages toggleable repeat workers for StickyRepeat (Mode 5)
#[derive(Default)]
pub struct StickyRepeatManager {
    workers: HashMap<String, RepeatWorker>,
}

impl StickyRepeatManager {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Toggles sticky repeat for `id`.
    /// If currently running, stops it and returns `false`.
    /// If stopped, starts it and returns `true`.
    pub fn toggle<F>(
        &mut self,
        id: &str,
        keys: ParsedKeySequence,
        delay_ms: u32,
        randomize: bool,
        action: F,
    ) -> bool
    where
        F: FnMut(&ParsedKeySequence) + Send + 'static,
    {
        if let Some(mut worker) = self.workers.remove(id) {
            worker.stop();
            false
        } else {
            let worker = RepeatWorker::start(keys, delay_ms, randomize, action);
            self.workers.insert(id.to_string(), worker);
            true
        }
    }

    pub fn stop(&mut self, id: &str) {
        if let Some(mut worker) = self.workers.remove(id) {
            worker.stop();
        }
    }

    pub fn stop_all(&mut self) {
        for (_, mut worker) in self.workers.drain() {
            worker.stop();
        }
    }

    #[must_use]
    pub fn is_active(&self, id: &str) -> bool {
        self.workers.contains_key(id)
    }
}
