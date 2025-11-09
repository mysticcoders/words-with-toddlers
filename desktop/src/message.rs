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
    /// Triggered periodically to toggle cursor visibility
    ToggleCursor,
    /// Navigate to settings screen
    NavigateToSettings,
    /// Navigate to welcome screen
    NavigateToWelcome,
    /// Navigate to main (discovery mode) screen
    NavigateToMain,
    /// Start visual challenge mode
    StartVisualChallenge,
    /// Start audio challenge mode
    StartAudioChallenge,
    /// Replay the current word (audio mode)
    ReplayWord,
    /// Select a sound by name
    SelectSound(String),
    /// Toggle typewriter mode
    ToggleTypewriterMode(bool),
    /// Check if typed word is correct (challenge mode)
    CheckTypedWord,
    /// Finish celebration and load next word
    FinishCelebration,
    /// Exit challenge mode
    ExitChallenge,
}
