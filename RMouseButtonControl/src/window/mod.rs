pub mod fallback;
pub mod linux_x11;

pub use fallback::StaticWindowDetector;
pub use linux_x11::LinuxX11WindowDetector;

pub trait CurrentWindow: Send + Sync {
    fn get_foreground_window(&self) -> String;
}
