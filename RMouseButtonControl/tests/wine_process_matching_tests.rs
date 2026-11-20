use rmousebuttoncontrol::matcher::ProfileMatcher;
use rmousebuttoncontrol::models::Profile;

fn profile(process: &str, checked: bool) -> Profile {
    Profile::new(process, process, checked)
}

fn should_skip(foreground: &str, p: &Profile) -> bool {
    ProfileMatcher::should_skip_profile(p, foreground)
}

#[test]
fn wine_game_matched_via_command_line_not_skipped() {
    // /exe points at the wine loader; the real exe is only in the command line (C# issue #29 parity)
    let foreground = "/opt/wine/bin/wine64-preloader Z:\\games\\MyGame\\game.exe -windowed";
    assert!(!should_skip(foreground, &profile("game.exe", true)));
}

#[test]
fn native_app_matched_via_exe_path_not_skipped() {
    let foreground = "/usr/lib/firefox/firefox /usr/lib/firefox/firefox";
    assert!(!should_skip(foreground, &profile("firefox", true)));
}

#[test]
fn different_process_is_skipped() {
    let foreground = "/usr/lib/firefox/firefox";
    assert!(should_skip(foreground, &profile("game.exe", true)));
}

#[test]
fn wildcard_profile_always_matches() {
    let foreground = "/usr/lib/firefox/firefox";
    assert!(!should_skip(foreground, &profile("*", true)));
}

#[test]
fn unchecked_profile_is_skipped() {
    let foreground = "/opt/wine/bin/wine64-preloader Z:\\games\\game.exe";
    assert!(should_skip(foreground, &profile("game.exe", false)));
}
