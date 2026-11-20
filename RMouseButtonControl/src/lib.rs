#![forbid(unsafe_code)]

pub mod engine;
pub mod hook;
pub mod matcher;
pub mod models;
pub mod parser;
pub mod simulation;
pub mod storage;
pub mod tray;
pub mod window;

pub use engine::{CompiledProfile, EventDispatcher, EventDisposition, ProfilesCache};
pub use hook::MouseHookService;
pub use matcher::ProfileMatcher;
pub use models::{ActionType, ButtonMapping, MouseButton, MouseButtonState, Profile};
pub use parser::{parse_keys, KeyOrButton, ParsedKey, ParsedKeySequence};
pub use simulation::{
    MockSimulator, RepeatWorker, Simulator, StickyHoldManager, StickyRepeatManager,
    UInputSimulator, UserspaceSimulator,
};
pub use storage::SqliteProfileLoader;
pub use tray::TrayManager;
pub use window::{CurrentWindow, LinuxX11WindowDetector, StaticWindowDetector};
