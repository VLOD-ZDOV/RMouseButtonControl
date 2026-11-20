use super::cache::{CompiledProfile, ProfilesCache};
use crate::matcher::ProfileMatcher;
use crate::models::{ActionType, ButtonMapping, MouseButton, MouseButtonState};
use crate::simulation::{RepeatWorker, Simulator, StickyHoldManager, StickyRepeatManager};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventDisposition {
    pub should_suppress: bool,
    pub matched_profiles: usize,
}

pub struct EventDispatcher<S: Simulator + ?Sized> {
    cache: Arc<ProfilesCache>,
    simulator: Arc<S>,
    hold_manager: Mutex<StickyHoldManager>,
    repeat_manager: Mutex<StickyRepeatManager>,
    active_repeater_workers: Mutex<HashMap<String, RepeatWorker>>,
    enabled: Arc<AtomicBool>,
}

impl<S: Simulator + ?Sized + 'static> EventDispatcher<S> {
    pub fn new(cache: Arc<ProfilesCache>, simulator: Arc<S>, enabled: Arc<AtomicBool>) -> Self {
        Self {
            cache,
            simulator,
            hold_manager: Mutex::new(StickyHoldManager::new()),
            repeat_manager: Mutex::new(StickyRepeatManager::new()),
            active_repeater_workers: Mutex::new(HashMap::new()),
            enabled,
        }
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Acquire)
    }

    pub fn set_enabled(&self, val: bool) {
        self.enabled.store(val, Ordering::Release);
    }

    pub fn toggle_enabled(&self) -> bool {
        let prev = self.enabled.fetch_xor(true, Ordering::AcqRel);
        !prev
    }

    /// Evaluates whether the original mouse event should be suppressed from the OS,
    /// 1:1 with C# MouseListenerService.ShouldSuppressEvent
    pub fn should_suppress_event(&self, button: MouseButton, foreground_window: &str) -> bool {
        if !self.is_enabled() {
            return false;
        }
        let profiles = self.cache.get_profiles();
        profiles.iter().any(|p| {
            if ProfileMatcher::should_skip_profile(&p.profile, foreground_window) {
                return false;
            }
            p.get_mapping(button).block_original_mouse_input
        })
    }

    /// Processes a mouse event across all matching profiles, 1:1 with KeyboardSimulatorWorker
    pub fn process_event(
        &self,
        button: MouseButton,
        state: MouseButtonState,
        foreground_window: &str,
    ) -> EventDisposition {
        if !self.is_enabled() {
            return EventDisposition {
                should_suppress: false,
                matched_profiles: 0,
            };
        }

        let profiles = self.cache.get_profiles();
        let mut matched_count = 0;
        let mut should_suppress = false;

        for compiled in &profiles {
            if ProfileMatcher::should_skip_profile(&compiled.profile, foreground_window) {
                continue;
            }

            matched_count += 1;
            let mapping = compiled.get_mapping(button);
            if mapping.block_original_mouse_input {
                should_suppress = true;
            }

            self.route_button_mapping(compiled, button, mapping, state);
        }

        EventDisposition {
            should_suppress,
            matched_profiles: matched_count,
        }
    }

    fn route_button_mapping(
        &self,
        profile: &CompiledProfile,
        button: MouseButton,
        mapping: &ButtonMapping,
        state: MouseButtonState,
    ) {
        match mapping.action {
            ActionType::Nothing | ActionType::Disabled => {
                // Passthrough or intentionally disabled (only suppression matters)
            }
            ActionType::RightClick => {
                match state {
                    MouseButtonState::Pressed => {
                        self.simulator.press_mouse(MouseButton::MouseButton2);
                    }
                    MouseButtonState::Released => {
                        self.simulator.release_mouse(MouseButton::MouseButton2);
                    }
                }
            }
            ActionType::AsMouseButtonPressed => {
                if state == MouseButtonState::Pressed {
                    if let Some(seq) = profile.get_parsed_sequence(button) {
                        self.simulator.tap_sequence(seq, None);
                    }
                }
            }
            ActionType::AsMouseButtonReleased => {
                if state == MouseButtonState::Released {
                    if let Some(seq) = profile.get_parsed_sequence(button) {
                        self.simulator.tap_sequence(seq, None);
                    }
                }
            }
            ActionType::DuringMousePressAndRelease => {
                if let Some(seq) = profile.get_parsed_sequence(button) {
                    match state {
                        MouseButtonState::Pressed => self.simulator.press_sequence(seq),
                        MouseButtonState::Released => self.simulator.release_sequence(seq),
                    }
                }
            }
            ActionType::RepeatedWhileButtonDown => {
                let id = format!("{}:{}", profile.profile.name, button.name());
                match state {
                    MouseButtonState::Pressed => {
                        if let Some(seq) = profile.get_parsed_sequence(button) {
                            let delay = mapping.auto_repeat_delay.unwrap_or(33);
                            let randomize = mapping
                                .auto_repeat_randomize_delay_enabled
                                .unwrap_or(false);
                            let sim = Arc::clone(&self.simulator);
                            let worker = RepeatWorker::start(
                                seq.clone(),
                                delay,
                                randomize,
                                move |keys| {
                                    sim.tap_sequence(keys, None);
                                },
                            );
                            if let Ok(mut workers) = self.active_repeater_workers.lock() {
                                workers.insert(id, worker);
                            }
                        }
                    }
                    MouseButtonState::Released => {
                        if let Ok(mut workers) = self.active_repeater_workers.lock() {
                            if let Some(mut worker) = workers.remove(&id) {
                                worker.stop();
                            }
                        }
                    }
                }
            }
            ActionType::StickyRepeat => {
                if state == MouseButtonState::Pressed {
                    if let Some(seq) = profile.get_parsed_sequence(button) {
                        let id = format!("{}:{}", profile.profile.name, button.name());
                        let delay = mapping.auto_repeat_delay.unwrap_or(33);
                        let randomize = mapping
                            .auto_repeat_randomize_delay_enabled
                            .unwrap_or(false);
                        let sim = Arc::clone(&self.simulator);

                        if let Ok(mut manager) = self.repeat_manager.lock() {
                            manager.toggle(&id, seq.clone(), delay, randomize, move |keys| {
                                sim.tap_sequence(keys, None);
                            });
                        }
                    }
                }
            }
            ActionType::StickyHold => {
                if state == MouseButtonState::Pressed {
                    if let Some(seq) = profile.get_parsed_sequence(button) {
                        let id = format!("{}:{}", profile.profile.name, button.name());
                        if let Ok(mut manager) = self.hold_manager.lock() {
                            let became_held = manager.toggle(&id);
                            if became_held {
                                self.simulator.press_sequence(seq);
                            } else {
                                self.simulator.release_sequence(seq);
                            }
                        }
                    }
                }
            }
        }
    }
}
