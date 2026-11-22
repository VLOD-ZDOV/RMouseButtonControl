use crate::models::{ButtonMapping, MouseButton, Profile};
use crate::parser::{parse_keys, ParsedKeySequence};
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct CompiledProfile {
    pub profile: Profile,
    /// Pre-parsed key sequences for hot-path zero-allocation lookup
    pub parsed_keys: HashMap<MouseButton, ParsedKeySequence>,
}

impl CompiledProfile {
    #[must_use]
    pub fn new(profile: Profile) -> Self {
        let mut parsed_keys = HashMap::new();

        let buttons = [
            MouseButton::MouseButton1,
            MouseButton::MouseButton2,
            MouseButton::MouseButton3,
            MouseButton::MouseButton4,
            MouseButton::MouseButton5,
            MouseButton::MouseWheelUp,
            MouseButton::MouseWheelDown,
            MouseButton::MouseWheelLeft,
            MouseButton::MouseWheelRight,
        ];

        for &btn in &buttons {
            let mapping = profile.get_mapping(btn);
            if let Some(ref keys_str) = mapping.keys {
                match parse_keys(keys_str) {
                    Ok(seq) => {
                        parsed_keys.insert(btn, seq);
                    }
                    Err(err) => {
                        eprintln!("[CompiledProfile] Warning: failed to parse macro for profile '{}', button {:?}: {err}", profile.name, btn);
                    }
                }
            }
        }

        Self {
            profile,
            parsed_keys,
        }
    }

    #[must_use]
    pub fn get_mapping(&self, btn: MouseButton) -> &ButtonMapping {
        self.profile.get_mapping(btn)
    }

    #[must_use]
    pub fn get_parsed_sequence(&self, btn: MouseButton) -> Option<&ParsedKeySequence> {
        self.parsed_keys.get(&btn)
    }
}

pub struct ProfilesCache {
    profiles: RwLock<Vec<CompiledProfile>>,
}

impl Default for ProfilesCache {
    fn default() -> Self {
        Self {
            profiles: RwLock::new(vec![CompiledProfile::new(Profile::default())]),
        }
    }
}

impl ProfilesCache {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_profiles(&self, new_profiles: Vec<Profile>) {
        let compiled = new_profiles
            .into_iter()
            .map(CompiledProfile::new)
            .collect();
        if let Ok(mut lock) = self.profiles.write() {
            *lock = compiled;
        }
    }

    #[must_use]
    pub fn get_profiles(&self) -> Vec<CompiledProfile> {
        self.profiles.read().map(|p| p.clone()).unwrap_or_default()
    }
}
