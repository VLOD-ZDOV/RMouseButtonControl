use crate::parser::ParsedKeySequence;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct RepeatWorker {
    stop_signal: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl RepeatWorker {
    /// Computes sleep time with optional 10% jitter (minimum 10ms to prevent CPU busy-spinning)
    #[must_use]
    pub fn compute_delay_ms(base_delay: u32, randomize: bool) -> u64 {
        let base = base_delay.max(10);
        if !randomize {
            return base as u64;
        }

        let rand_percent = fastrand::f64() * 0.1; // 0.0 .. 0.10
        let delta = (base as f64 * rand_percent).round() as u32;

        let final_delay = if fastrand::bool() {
            base.saturating_add(delta)
        } else {
            base.saturating_sub(delta).max(10)
        };

        final_delay as u64
    }

    /// Starts a background repetition loop that executes `action` on each tick
    pub fn start<F>(
        keys: ParsedKeySequence,
        base_delay_ms: u32,
        randomize: bool,
        mut action: F,
    ) -> Self
    where
        F: FnMut(&ParsedKeySequence) + Send + 'static,
    {
        let stop_signal = Arc::new(AtomicBool::new(false));
        let stop_clone = Arc::clone(&stop_signal);

        let handle = thread::spawn(move || {
            while !stop_clone.load(Ordering::Acquire) {
                let delay = Self::compute_delay_ms(base_delay_ms, randomize);
                thread::sleep(Duration::from_millis(delay));

                if stop_clone.load(Ordering::Acquire) {
                    break;
                }

                action(&keys);
            }
        });

        Self {
            stop_signal,
            handle: Some(handle),
        }
    }

    pub fn stop(&mut self) {
        self.stop_signal.store(true, Ordering::Release);
        self.handle.take();
    }

    #[must_use]
    pub fn is_running(&self) -> bool {
        !self.stop_signal.load(Ordering::Acquire) && self.handle.is_some()
    }
}

impl Drop for RepeatWorker {
    fn drop(&mut self) {
        self.stop();
    }
}
