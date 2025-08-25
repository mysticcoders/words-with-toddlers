mod app;
mod letter;
mod message;
mod utils;

use app::WordsWithToddlers;
use iced::{window, Size};

/// Entry point for the Words with Toddlers application
///
/// This creates a toddler-friendly typing application with:
/// - Large, colorful letters
/// - Support for up to 25 characters
/// - Fullscreen capability
/// - Always-on-top window setting
fn main() -> iced::Result {
    iced::application(
        "Words with Toddlers",
        WordsWithToddlers::update,
        WordsWithToddlers::view,
    )
    .window(create_window_settings())
    .subscription(WordsWithToddlers::subscription)
    .theme(WordsWithToddlers::theme)
    .run_with(WordsWithToddlers::new)
}

/// Creates the window settings for the application
fn create_window_settings() -> window::Settings {
    window::Settings {
        size: Size::new(1200.0, 800.0),
        position: window::Position::Centered,
        decorations: true,
        resizable: true,
        transparent: false,
        level: window::Level::AlwaysOnTop,
        ..Default::default()
    }
}

// Note on macOS System Key Capture:
//
// To capture system keyboard shortcuts (like Cmd+Tab) on macOS, you would need:
//
// 1. **Accessibility Permissions**: Your app needs to be granted accessibility
//    permissions in System Preferences > Security & Privacy > Privacy > Accessibility
//
// 2. **CGEventTap API**: Use the Core Graphics Event Tap API through the
//    `core-graphics` crate. Example:
//    ```rust
//    use core_graphics::event::{CGEventTap, CGEventTapLocation, CGEventTapPlacement};
//    ```
//
// 3. **Root Access or Entitlements**: Some system shortcuts can only be captured
//    when running as root or with specific entitlements.
//
// 4. **Alternative: macOS Guided Access**: For toddler safety without code changes:
//    - Go to System Settings → Accessibility → Guided Access
//    - Enable it and set a passcode
//    - Triple-click Touch ID/power button to lock the Mac to just this app
//
// Implementation would require adding these dependencies to Cargo.toml:
// ```toml
// core-graphics = "0.23"
// core-foundation = "0.9"
// ```
//
// However, Apple's security model intentionally makes it difficult to capture
// system shortcuts to prevent malicious applications. The recommended approach
// for a toddler-safe environment is using built-in parental controls or
// Guided Access mode.
