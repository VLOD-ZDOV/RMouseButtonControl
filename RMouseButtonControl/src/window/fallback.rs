use super::CurrentWindow;
use std::sync::RwLock;

pub struct StaticWindowDetector {
    current: RwLock<String>,
}

impl StaticWindowDetector {
    #[must_use]
    pub fn new(initial: impl Into<String>) -> Self {
        Self {
            current: RwLock::new(initial.into()),
        }
    }

    pub fn set_foreground_window(&self, window: impl Into<String>) {
        if let Ok(mut lock) = self.current.write() {
            *lock = window.into();
        }
    }
}

impl CurrentWindow for StaticWindowDetector {
    fn get_foreground_window(&self) -> String {
        self.current.read().map(|s| s.clone()).unwrap_or_else(|_| "*".to_string())
    }
}
