#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub mod permission;
mod types;

pub use types::*;

#[cfg(target_os = "macos")]
pub use macos::KioskMode;

#[cfg(not(target_os = "macos"))]
pub struct KioskMode;

#[cfg(not(target_os = "macos"))]
impl KioskMode {
    pub fn start() -> Result<Self, KioskError> {
        Err(KioskError::NotSupported)
    }

    pub fn stop(&mut self) {}

    pub fn is_running(&self) -> bool {
        false
    }
}

#[cfg(not(target_os = "macos"))]
pub mod permission {
    pub fn has_accessibility_permission() -> bool {
        false
    }

    pub fn request_accessibility_permission() -> bool {
        false
    }

    pub fn open_accessibility_preferences() {}
}
