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
    #[allow(dead_code)]
    NavigateToMain,
    /// Start visual challenge mode
    StartVisualChallenge,
    /// Start audio challenge mode
    StartAudioChallenge,
    /// Replay the current word (audio mode)
    ReplayWord,
    /// Select a sound by name
    SelectSound(String),
    /// Toggle uppercase/lowercase display
    ToggleUppercase(bool),
    /// Check if typed word is correct (challenge mode)
    CheckTypedWord,
    /// Finish celebration and load next word
    FinishCelebration,
    /// Exit challenge mode
    ExitChallenge,
    /// Start tic-tac-toe game
    StartTicTacToe,
    /// Make a move in tic-tac-toe at position (0-8)
    TicTacToeMove(usize),
    /// Reset the tic-tac-toe game
    ResetTicTacToe,
    /// Exit tic-tac-toe game
    ExitTicTacToe,
}
