use crate::models::Profile;
use crate::window::CurrentWindow;

pub struct ProfileMatcher;

impl ProfileMatcher {
    /// 1:1 contract with C# SkipProfileLinux:
    /// - If profile is not checked -> skip (true)
    /// - If profile.process is "*" -> match (false)
    /// - If foreground_window contains profile.process -> match (false)
    /// - If foreground_window is "*" -> match (false)
    /// - Otherwise -> skip (true)
    #[must_use]
    pub fn should_skip_profile(profile: &Profile, foreground_window: &str) -> bool {
        if !profile.checked {
            return true;
        }

        if profile.process == "*" {
            return false;
        }

        if foreground_window == "*" {
            return false;
        }

        if foreground_window.contains(&profile.process) {
            return false;
        }

        true
    }

    #[must_use]
    pub fn matches(profile: &Profile, foreground_window: &str) -> bool {
        !Self::should_skip_profile(profile, foreground_window)
    }

    #[must_use]
    pub fn matches_with_detector<W: CurrentWindow>(profile: &Profile, detector: &W) -> bool {
        let foreground = detector.get_foreground_window();
        Self::matches(profile, &foreground)
    }
}
