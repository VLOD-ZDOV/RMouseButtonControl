use super::CurrentWindow;

#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use x11rb::connection::Connection;
#[cfg(target_os = "linux")]
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt, GetPropertyReply};

pub struct LinuxX11WindowDetector;

impl LinuxX11WindowDetector {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Builds the identity string for matching, 1:1 with C# GetCurrentWindowLinuxX11:
    /// combines `/proc/<pid>/exe` with `/proc/<pid>/cmdline` (flattened with spaces)
    /// to support Wine/Proton games where `/exe` is wine-preloader but cmdline contains `game.exe`.
    #[cfg(target_os = "linux")]
    pub fn get_identity_from_pid(pid: u32) -> String {
        let exe = fs::read_link(format!("/proc/{pid}/exe"))
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();

        let cmdline = fs::read(format!("/proc/{pid}/cmdline"))
            .map(|bytes| {
                let s = String::from_utf8_lossy(&bytes);
                s.replace('\0', " ").trim().to_string()
            })
            .unwrap_or_default();

        if cmdline.is_empty() {
            exe
        } else if exe.is_empty() {
            cmdline
        } else {
            format!("{exe} {cmdline}")
        }
    }

    #[cfg(target_os = "linux")]
    fn get_active_window_pid() -> Option<u32> {
        let (conn, screen_num) = x11rb::connect(None).ok()?;
        let screen = conn.setup().roots.get(screen_num)?;
        let root = screen.root;

        let active_win_cookie = conn.intern_atom(false, b"_NET_ACTIVE_WINDOW").ok()?;
        let pid_cookie = conn.intern_atom(false, b"_NET_WM_PID").ok()?;

        let net_active_window = active_win_cookie.reply().ok()?.atom;
        let net_wm_pid = pid_cookie.reply().ok()?.atom;

        let none_atom = u32::from(AtomEnum::NONE);
        if net_active_window == none_atom || net_wm_pid == none_atom {
            return None;
        }

        // 1. Get _NET_ACTIVE_WINDOW from root
        let prop = conn
            .get_property(
                false,
                root,
                net_active_window,
                AtomEnum::WINDOW,
                0,
                1,
            )
            .ok()?
            .reply()
            .ok()?;

        let active_window = parse_window_from_property(&prop)?;
        if active_window == 0 {
            return None;
        }

        // 2. Get _NET_WM_PID from active window
        let pid_prop = conn
            .get_property(
                false,
                active_window,
                net_wm_pid,
                AtomEnum::CARDINAL,
                0,
                1,
            )
            .ok()?
            .reply()
            .ok()?;

        parse_pid_from_property(&pid_prop)
    }
}

#[cfg(target_os = "linux")]
fn parse_window_from_property(reply: &GetPropertyReply) -> Option<u32> {
    if reply.format == 32 && reply.value.len() >= 4 {
        let b = &reply.value[0..4];
        Some(u32::from_ne_bytes([b[0], b[1], b[2], b[3]]))
    } else {
        None
    }
}

#[cfg(target_os = "linux")]
fn parse_pid_from_property(reply: &GetPropertyReply) -> Option<u32> {
    if reply.format == 32 && reply.value.len() >= 4 {
        let b = &reply.value[0..4];
        Some(u32::from_ne_bytes([b[0], b[1], b[2], b[3]]))
    } else {
        None
    }
}

impl Default for LinuxX11WindowDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrentWindow for LinuxX11WindowDetector {
    fn get_foreground_window(&self) -> String {
        #[cfg(target_os = "linux")]
        {
            match Self::get_active_window_pid() {
                Some(pid) => Self::get_identity_from_pid(pid),
                None => "*".to_string(), // Fallback to matching everything if no X11 window found
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            "*".to_string()
        }
    }
}
