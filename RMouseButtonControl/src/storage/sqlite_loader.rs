use crate::models::{ActionType, ButtonMapping, MouseButton, Profile};
use rusqlite::{Connection, Result};
use std::path::Path;

pub struct SqliteProfileLoader;

impl SqliteProfileLoader {
    pub fn load_from_db<P: AsRef<Path>>(path: P) -> Result<Vec<Profile>> {
        let conn = Connection::open(path)?;

        let mut stmt = conn.prepare(
            "SELECT Id, Name, Description, Process, WindowCaption, WindowClass, ParentClass, MatchType, Checked
             FROM Profiles ORDER BY DisplayPriority ASC",
        )?;

        let profile_rows = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let name: String = row.get(1)?;
            let description: String = row.get(2)?;
            let process: String = row.get(3)?;
            let window_caption: String = row.get(4)?;
            let window_class: String = row.get(5)?;
            let parent_class: String = row.get(6)?;
            let match_type: String = row.get(7)?;
            let checked_val: i32 = row.get(8)?;

            Ok((
                id,
                Profile {
                    name,
                    description,
                    process,
                    window_caption,
                    window_class,
                    parent_class,
                    match_type,
                    checked: checked_val != 0,
                    ..Profile::default()
                },
            ))
        })?;

        let mut profiles = Vec::new();
        for p in profile_rows {
            let (profile_id, mut profile) = p?;

            // Load active button mappings (Selected = 1) for this profile
            let mut map_stmt = conn.prepare(
                "SELECT MouseButton, Keys, SimulatedKeystrokeType, AutoRepeatDelay, AutoRepeatRandomizeDelayEnabled, BlockOriginalMouseInput, Discriminator
                 FROM ButtonMappings WHERE ProfileId = ? AND Selected = 1",
            )?;

            let mapping_rows = map_stmt.query_map([profile_id], |row| {
                let mb_index: i32 = row.get(0)?;
                let keys: Option<String> = row.get(1)?;
                let sim_type: Option<i32> = row.get(2)?;
                let delay: Option<u32> = row.get(3)?;
                let rand_enabled: Option<i32> = row.get(4)?;
                let block_input: Option<i32> = row.get(5)?;
                let discriminator: String = row.get(6)?;

                Ok((
                    mb_index,
                    keys,
                    sim_type,
                    delay,
                    rand_enabled.map(|v| v != 0),
                    block_input.map(|v| v != 0).unwrap_or(false),
                    discriminator,
                ))
            })?;

            for m in mapping_rows {
                let (mb_idx, keys, sim_type, delay, rand_en, block, discr) = m?;
                let target_button = match mb_idx {
                    0 => MouseButton::MouseButton1,
                    1 => MouseButton::MouseButton2,
                    2 => MouseButton::MouseButton3,
                    3 => MouseButton::MouseButton4,
                    4 => MouseButton::MouseButton5,
                    5 => MouseWheelUp,
                    6 => MouseWheelDown,
                    7 => MouseWheelLeft,
                    8 => MouseWheelRight,
                    _ => continue,
                };

                let action = match discr.as_str() {
                    "DisabledMapping" => ActionType::Disabled,
                    "RightClick" => ActionType::RightClick,
                    "SimulatedKeystroke" => match sim_type {
                        Some(0) => ActionType::AsMouseButtonPressed,
                        Some(1) => ActionType::DuringMousePressAndRelease,
                        Some(2) | Some(4) => ActionType::AsMouseButtonPressed,
                        Some(3) | Some(5) => ActionType::AsMouseButtonReleased,
                        Some(6) => ActionType::RepeatedWhileButtonDown,
                        Some(7) => ActionType::StickyHold,
                        Some(8) => ActionType::StickyRepeat,
                        _ => ActionType::AsMouseButtonPressed,
                    },
                    _ => ActionType::Nothing,
                };

                let mapping = ButtonMapping {
                    action,
                    keys,
                    auto_repeat_delay: delay.or(Some(33)),
                    auto_repeat_randomize_delay_enabled: rand_en.or(Some(false)),
                    block_original_mouse_input: block,
                };

                profile.set_mapping(target_button, mapping);
            }

            profiles.push(profile);
        }

        Ok(profiles)
    }

    #[must_use]
    pub fn default_db_path() -> Option<std::path::PathBuf> {
        #[cfg(target_os = "windows")]
        {
            if let Ok(appdata) = std::env::var("APPDATA") {
                let base = std::path::PathBuf::from(appdata);
                let new_path = base.join("RMouseButtonControl").join("RMouseButtonControl.db");
                if new_path.exists() {
                    return Some(new_path);
                }
                let old_path = base.join("YMouseButtonControl").join("YMouseButtonControl.db");
                if old_path.exists() {
                    return Some(old_path);
                }
                return Some(new_path);
            }
        }
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).ok()?;
        let base = std::path::PathBuf::from(home);
        let new_path = base.join(".config").join("RMouseButtonControl").join("RMouseButtonControl.db");
        if new_path.exists() {
            return Some(new_path);
        }
        let old_path = base.join(".config").join("YMouseButtonControl").join("YMouseButtonControl.db");
        if old_path.exists() {
            return Some(old_path);
        }
        Some(new_path)
    }
}

use MouseButton::*;
