#[cfg(target_os = "linux")]
use ksni::blocking::TrayMethods;
#[cfg(target_os = "linux")]
use ksni::menu::{CheckmarkItem, MenuItem, StandardItem, SubMenu};
#[cfg(target_os = "linux")]
use ksni::Tray;
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::path::{Path, PathBuf};
#[cfg(target_os = "linux")]
use std::process::Command;
#[cfg(target_os = "linux")]
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use crate::engine::ProfilesCache;
#[cfg(target_os = "linux")]
use crate::models::{ActionType, MouseButton};
#[cfg(target_os = "linux")]
use crate::storage::SqliteProfileLoader;

#[cfg(target_os = "linux")]
struct RMouseTray {
    cache: Arc<ProfilesCache>,
    enabled: Arc<AtomicBool>,
}

#[cfg(target_os = "linux")]
impl RMouseTray {
    fn autostart_path() -> Option<PathBuf> {
        let home = std::env::var("HOME").ok()?;
        Some(PathBuf::from(home).join(".config").join("autostart").join("RMouseButtonControl.desktop"))
    }

    fn is_autostart_enabled() -> bool {
        Self::autostart_path().map(|p| p.exists()).unwrap_or(false)
    }

    fn toggle_autostart() {
        if let Some(path) = Self::autostart_path() {
            if path.exists() {
                let _ = fs::remove_file(&path);
                println!("[Tray] Autostart disabled");
            } else {
                if let Some(parent) = path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
                let desktop_entry = format!(
                    "[Desktop Entry]\n\
                     Type=Application\n\
                     Exec={home}/.local/bin/rmousebuttoncontrol\n\
                     Hidden=false\n\
                     NoDisplay=false\n\
                     X-GNOME-Autostart-enabled=true\n\
                     Name=RMouseButtonControl\n\
                     Comment=RMouseButtonControl Daemon\n"
                );
                let _ = fs::write(&path, desktop_entry);
                println!("[Tray] Autostart enabled");
            }
        }
    }

    fn get_preferred_gui() -> String {
        if let Some(path) = SqliteProfileLoader::default_db_path() {
            if let Ok(conn) = rusqlite::Connection::open(&path) {
                if let Ok(val) = conn.query_row(
                    "SELECT StringValue FROM Settings WHERE Name = 'PreferredGui'",
                    [],
                    |row| row.get::<_, String>(0),
                ) {
                    return val;
                }
            }
        }
        "qt".to_string()
    }

    fn set_preferred_gui(gui: &str) {
        if let Some(path) = SqliteProfileLoader::default_db_path() {
            if let Ok(conn) = rusqlite::Connection::open(&path) {
                let updated = conn.execute(
                    "UPDATE Settings SET StringValue = ?1 WHERE Name = 'PreferredGui'",
                    [gui],
                ).unwrap_or(0);
                if updated == 0 {
                    let _ = conn.execute(
                        "INSERT INTO Settings (Name, Discriminator, StringValue) VALUES ('PreferredGui', 'SettingString', ?1)",
                        [gui],
                    );
                }
            }
        }
    }

    fn launch_qt_gui() -> bool {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        let candidates = [
            format!("{home}/.local/bin/rmouse-gui"),
            format!("{home}/.local/bin/rmousebuttoncontrol-gui"),
            "rmouse-gui".to_string(),
        ];
        for path in &candidates {
            if Path::new(path).exists() || path == "rmouse-gui" {
                if let Ok(_) = Command::new(path).spawn() {
                    println!("[Tray] Launched Qt GUI from {path}");
                    return true;
                }
            }
        }
        false
    }

    fn launch_csharp_gui() -> bool {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        let candidates = [
            format!("{home}/.local/bin/rmouse-settings"),
            format!("{home}/.local/bin/ymouse-settings"),
            format!("{home}/.local/lib/ymouse/YMouseButtonControl"),
            "rmouse-settings".to_string(),
            "ymouse-settings".to_string(),
        ];
        for path in &candidates {
            if Path::new(path).exists() || path == "ymouse-settings" {
                if let Ok(_) = Command::new(path).arg("--settings").spawn() {
                    println!("[Tray] Launched C# Avalonia GUI from {path}");
                    return true;
                }
            }
        }
        false
    }

    fn launch_setup() {
        let pref = Self::get_preferred_gui();
        if pref == "csharp" || pref == "avalonia" {
            if !Self::launch_csharp_gui() {
                Self::launch_qt_gui();
            }
        } else {
            if !Self::launch_qt_gui() {
                Self::launch_csharp_gui();
            }
        }
    }
}

#[cfg(target_os = "linux")]
impl Tray for RMouseTray {
    fn id(&self) -> String {
        "rmousebuttoncontrol".into()
    }

    fn icon_name(&self) -> String {
        if self.enabled.load(Ordering::Acquire) {
            "input-mouse".into()
        } else {
            "dialog-warning".into()
        }
    }

    fn title(&self) -> String {
        "RMouseButtonControl".into()
    }

    fn tool_tip(&self) -> ksni::ToolTip {
        let status = if self.enabled.load(Ordering::Acquire) {
            "Active"
        } else {
            "Disabled (Paused)"
        };
        ksni::ToolTip {
            title: "RMouseButtonControl".into(),
            description: format!("Status: {status} (Rust Core Engine)"),
            icon_name: "input-mouse".into(),
            ..Default::default()
        }
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let is_enabled = self.enabled.load(Ordering::Acquire);
        let enabled_clone = Arc::clone(&self.enabled);
        let cache_clone = Arc::clone(&self.cache);

        // Build profiles submenu
        let profiles = self.cache.get_profiles();
        let mut profile_items: Vec<MenuItem<Self>> = Vec::new();

        for cp in &profiles {
            let p = &cp.profile;
            let check_icon = if p.checked { "[✓] " } else { "[ ] " };
            profile_items.push(
                StandardItem {
                    label: format!("{check_icon}{} ({})", p.name, p.process),
                    enabled: false,
                    ..Default::default()
                }
                .into(),
            );

            for &btn in &[
                MouseButton::MouseButton1,
                MouseButton::MouseButton2,
                MouseButton::MouseButton3,
                MouseButton::MouseButton4,
                MouseButton::MouseButton5,
            ] {
                let m = p.get_mapping(btn);
                if m.action != ActionType::Nothing {
                    let key_str = m.keys.as_deref().unwrap_or("none");
                    profile_items.push(
                        StandardItem {
                            label: format!("    • {}: {:?} -> {}", btn.name(), m.action, key_str),
                            enabled: false,
                            ..Default::default()
                        }
                        .into(),
                    );
                }
            }
        }

        let is_autostart = Self::is_autostart_enabled();

        vec![
            // 1. Setup window (opens GUI configurator)
            StandardItem {
                label: "Настройка... (Setup)".into(),
                icon_name: "configure".into(),
                activate: Box::new(|_| {
                    Self::launch_setup();
                }),
                ..Default::default()
            }
            .into(),

            // 1.5. GUI Selector Submenu
            SubMenu {
                label: "Интерфейс (GUI)".into(),
                icon_name: "preferences-desktop-theme".into(),
                submenu: vec![
                    StandardItem {
                        label: "Открыть Qt интерфейс (Быстрый)".into(),
                        icon_name: "application-x-executable".into(),
                        activate: Box::new(|_| {
                            let _ = Self::launch_qt_gui();
                        }),
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: "Открыть C# Avalonia интерфейс (Оригинал)".into(),
                        icon_name: "preferences-desktop-display".into(),
                        activate: Box::new(|_| {
                            let _ = Self::launch_csharp_gui();
                        }),
                        ..Default::default()
                    }
                    .into(),
                    MenuItem::Separator,
                    CheckmarkItem {
                        label: "По умолчанию: Qt".into(),
                        checked: Self::get_preferred_gui() != "csharp" && Self::get_preferred_gui() != "avalonia",
                        activate: Box::new(|_| {
                            Self::set_preferred_gui("qt");
                        }),
                        ..Default::default()
                    }
                    .into(),
                    CheckmarkItem {
                        label: "По умолчанию: C# Avalonia".into(),
                        checked: Self::get_preferred_gui() == "csharp" || Self::get_preferred_gui() == "avalonia",
                        activate: Box::new(|_| {
                            Self::set_preferred_gui("csharp");
                        }),
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into(),

            MenuItem::Separator,

            // 2. Enable / Disable toggle
            CheckmarkItem {
                label: "Включить перехват (Enable Mouse Control)".into(),
                checked: is_enabled,
                activate: Box::new(move |_| {
                    let new_val = !enabled_clone.fetch_xor(true, Ordering::AcqRel);
                    println!("[Tray] Mouse control toggled: enabled={new_val}");
                }),
                ..Default::default()
            }
            .into(),

            // 3. Autostart checkbox
            CheckmarkItem {
                label: "Запускать при старте системы (Run at startup)".into(),
                checked: is_autostart,
                activate: Box::new(|_| {
                    Self::toggle_autostart();
                }),
                ..Default::default()
            }
            .into(),

            MenuItem::Separator,

            // 4. Profiles submenu
            SubMenu {
                label: format!("Профили (Profiles: {})", profiles.len()),
                icon_name: "view-list-details".into(),
                submenu: profile_items,
                ..Default::default()
            }
            .into(),

            // 5. Reload profiles
            StandardItem {
                label: "Перезагрузить профили из БД (Reload Profiles)".into(),
                icon_name: "view-refresh".into(),
                activate: Box::new(move |_| {
                    if let Some(path) = SqliteProfileLoader::default_db_path() {
                        if let Ok(loaded) = SqliteProfileLoader::load_from_db(&path) {
                            println!("[Tray] Reloaded {} profiles from DB", loaded.len());
                            cache_clone.set_profiles(loaded);
                        }
                    }
                }),
                ..Default::default()
            }
            .into(),

            MenuItem::Separator,

            // 6. Exit
            StandardItem {
                label: "Выход (Exit)".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| {
                    println!("[Tray] Exiting RMouseButtonControl");
                    std::process::exit(0);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

pub struct TrayManager;

impl TrayManager {
    pub fn start(cache: Arc<ProfilesCache>, enabled: Arc<AtomicBool>) {
        #[cfg(target_os = "linux")]
        {
            let tray = RMouseTray { cache, enabled };
            match tray.spawn() {
                Ok(_handle) => {
                    println!("[Tray] StatusNotifierItem tray menu initialized");
                }
                Err(err) => {
                    eprintln!("[Tray] Warning: could not initialize system tray: {err}");
                }
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (cache, enabled);
        }
    }
}
