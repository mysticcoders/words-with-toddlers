use crate::config::ColorPalette;
use crate::kiosk_mode::KioskModeStatus;
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
    /// Show tic-tac-toe mode selection
    StartTicTacToe,
    /// Start one-player tic-tac-toe (vs computer)
    StartTicTacToeOnePlayer,
    /// Start two-player tic-tac-toe
    StartTicTacToeTwoPlayer,
    /// Make a move in tic-tac-toe at position (0-8)
    TicTacToeMove(usize),
    /// Computer makes its move
    TicTacToeComputerMove,
    /// Reset the tic-tac-toe game
    ResetTicTacToe,
    /// Exit tic-tac-toe game
    ExitTicTacToe,
    /// Toggle kiosk mode on/off
    ToggleKioskMode(bool),
    /// Kiosk mode status changed
    KioskModeStatusChanged(KioskModeStatus),
    /// Request accessibility permission for kiosk mode
    RequestAccessibilityPermission,
    /// Select a color palette for letter colors
    SelectColorPalette(ColorPalette),
}
