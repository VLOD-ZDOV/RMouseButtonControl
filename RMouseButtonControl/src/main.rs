#![forbid(unsafe_code)]

use rmousebuttoncontrol::engine::{EventDispatcher, ProfilesCache};
use rmousebuttoncontrol::hook::MouseHookService;
use rmousebuttoncontrol::models::{ActionType, MouseButton, Profile};
use rmousebuttoncontrol::simulation::{Simulator, UInputSimulator, UserspaceSimulator};
use rmousebuttoncontrol::storage::SqliteProfileLoader;
use rmousebuttoncontrol::window::{CurrentWindow, LinuxX11WindowDetector};
use std::sync::Arc;

fn main() {
    println!("=== RMouseButtonControl Daemon v0.1.0 ===");
    println!("High-performance safe input engine");

    // 1. Detect active window
    let detector = Arc::new(LinuxX11WindowDetector::new());
    let current_window = detector.get_foreground_window();
    println!("Active foreground window: {current_window}");

    // 2. Initialize simulator backend
    let (simulator, backend_name): (Arc<dyn Simulator>, &'static str) =
        match UInputSimulator::try_new() {
            Ok(uinput) => {
                println!("[Backend] Linux Kernel /dev/uinput virtual device initialized (Wayland & Anti-cheat compatible)");
                (Arc::new(uinput), "uinput")
            }
            Err(err) => {
                println!("[Backend] /dev/uinput not available ({err}), falling back to userspace simulator");
                (Arc::new(UserspaceSimulator::new()), "userspace")
            }
        };

    println!("Simulator backend active: {backend_name}");

    // 3. Setup profiles from existing YMouseButtonControl database if available
    let cache = Arc::new(ProfilesCache::new());

    let db_path = SqliteProfileLoader::default_db_path();
    let loaded_profiles = if let Some(ref path) = db_path {
        match SqliteProfileLoader::load_from_db(path) {
            Ok(profiles) => {
                println!("[Storage] Successfully loaded {} profiles from {:?}", profiles.len(), path);
                for p in &profiles {
                    println!("  - Profile '{}' (process: '{}', checked: {})", p.name, p.process, p.checked);
                    for &btn in &[
                        MouseButton::MouseButton1,
                        MouseButton::MouseButton2,
                        MouseButton::MouseButton3,
                        MouseButton::MouseButton4,
                        MouseButton::MouseButton5,
                    ] {
                        let m = p.get_mapping(btn);
                        if m.action != ActionType::Nothing {
                            println!("    * {}: {:?} (keys: {:?}, block: {})", btn.name(), m.action, m.keys, m.block_original_mouse_input);
                        }
                    }
                }
                profiles
            }
            Err(err) => {
                eprintln!("[Storage] Warning: Failed to load from DB ({err}), using default profile");
                vec![Profile::default()]
            }
        }
    } else {
        println!("[Storage] No existing database found, using default profile");
        vec![Profile::default()]
    };

    cache.set_profiles(loaded_profiles);

    let enabled = Arc::new(std::sync::atomic::AtomicBool::new(true));

    // 4. Initialize system tray icon in KDE / freedesktop with full menu
    rmousebuttoncontrol::tray::TrayManager::start(Arc::clone(&cache), Arc::clone(&enabled));

    // 4.5. Background SQLite database file watcher for instant profile hot-reload
    let cache_for_watch = Arc::clone(&cache);
    std::thread::spawn(move || {
        let mut last_mtime = None;
        if let Some(ref path) = SqliteProfileLoader::default_db_path() {
            if let Ok(meta) = std::fs::metadata(path) {
                last_mtime = meta.modified().ok();
            }
        }
        loop {
            std::thread::sleep(std::time::Duration::from_millis(500));
            if let Some(ref path) = SqliteProfileLoader::default_db_path() {
                if let Ok(meta) = std::fs::metadata(path) {
                    if let Ok(mtime) = meta.modified() {
                        if let Some(prev) = last_mtime {
                            if mtime > prev {
                                if let Ok(profiles) = SqliteProfileLoader::load_from_db(path) {
                                    println!("[Storage] Database changed, auto-reloaded {} profiles", profiles.len());
                                    cache_for_watch.set_profiles(profiles);
                                }
                            }
                        }
                        last_mtime = Some(mtime);
                    }
                }
            }
        }
    });

    // 5. Start mouse hook service
    let dispatcher = Arc::new(EventDispatcher::new(cache, simulator, enabled));
    let hook_service = MouseHookService::new(dispatcher, detector);

    println!("[Engine] Starting global mouse hook listener loop. Press Ctrl+C to exit.");
    if let Err(e) = hook_service.run() {
        eprintln!("[Engine] Error in mouse hook listener: {e:?}");
    }
}
