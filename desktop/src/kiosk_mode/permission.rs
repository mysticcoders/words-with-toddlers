use core_foundation::base::TCFType;
use core_foundation::boolean::CFBoolean;
use core_foundation::dictionary::CFMutableDictionary;
use core_foundation::string::CFString;
use std::process::Command;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrusted() -> bool;
    fn AXIsProcessTrustedWithOptions(options: core_foundation::dictionary::CFDictionaryRef) -> bool;
}

/// Checks if the current process has accessibility permission
pub fn has_accessibility_permission() -> bool {
    unsafe { AXIsProcessTrusted() }
}

/// Requests accessibility permission, showing the system dialog if needed.
/// Returns true if permission is already granted, false otherwise.
pub fn request_accessibility_permission() -> bool {
    unsafe {
        let key = CFString::new("AXTrustedCheckOptionPrompt");
        let value = CFBoolean::true_value();

        let mut dict = CFMutableDictionary::new();
        dict.add(&key.as_CFType(), &value.as_CFType());

        AXIsProcessTrustedWithOptions(dict.as_concrete_TypeRef())
    }
}

/// Opens the Accessibility preferences pane in System Settings
pub fn open_accessibility_preferences() {
    let _ = Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .spawn();
}
