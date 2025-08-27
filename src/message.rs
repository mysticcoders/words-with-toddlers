use iced::keyboard;

/// Application messages for handling user interactions
#[derive(Debug, Clone)]
pub enum Message {
    /// Triggered when a key is pressed
    KeyPressed(keyboard::Key),
    /// Triggered to toggle fullscreen mode
    ToggleFullscreen,
    /// Triggered when window opens to set it to always on top
    WindowOpened,
}
