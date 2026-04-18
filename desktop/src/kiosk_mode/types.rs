/// Errors that can occur when working with kiosk mode
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum KioskError {
    PermissionDenied,
    EventTapCreationFailed(String),
    AlreadyRunning,
    NotSupported,
}

impl std::fmt::Display for KioskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PermissionDenied => write!(f, "Accessibility permission not granted"),
            Self::EventTapCreationFailed(msg) => write!(f, "Failed to create event tap: {}", msg),
            Self::AlreadyRunning => write!(f, "Kiosk mode is already running"),
            Self::NotSupported => write!(f, "Kiosk mode is not supported on this platform"),
        }
    }
}

impl std::error::Error for KioskError {}

/// Status of kiosk mode
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum KioskModeStatus {
    Enabled,
    Disabled,
    Error(String),
    PermissionRequired,
}
