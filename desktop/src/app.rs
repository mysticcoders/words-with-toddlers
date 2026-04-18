use crate::celebration::Celebration;
use crate::config::ColorPalette;
use crate::dictionary::Dictionary;
use crate::discovered_word::DiscoveredWord;
use crate::grade_level::GradeLevel;
use crate::kiosk_mode::{KioskMode, KioskModeStatus};
use crate::letter::Letter;
use crate::message::Message;
use crate::session::Session;
use crate::tic_tac_toe::TicTacToe;
use crate::utils::color::hsl_to_rgb;
use crate::word_challenge::{ChallengeMode, WordChallenge};
use crate::word_list_loader::WordListLoader;
use iced::{
    alignment, event, exit, keyboard, mouse,
    widget::{
        button, canvas, column, container, row, scrollable, scrollable::Id as ScrollableId, stack,
        text, Row,
    },
    window, Color, Element, Event, Length, Point, Rectangle, Renderer, Subscription, Task, Theme,
};
use std::sync::{atomic::AtomicBool, Arc};

/// Represents the different screens in the application
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Welcome,
    Main,
    Settings,
    WordChallenge,
    TicTacToe,
}

/// Main application state for Words with Toddlers
pub struct WordsWithToddlers {
    letters: Vec<Letter>,
    discovered_words: Vec<DiscoveredWord>,
    dictionary: Dictionary,
    is_fullscreen: bool,
    letters_scroll_id: ScrollableId,
    has_started_typing: bool,
    cursor_visible: bool,
    sound_playing: Arc<AtomicBool>,
    current_screen: Screen,
    selected_sound: String,
    use_uppercase: bool,
    word_list_loader: WordListLoader,
    word_challenge: Option<WordChallenge>,
    celebration: Option<Celebration>,
    tic_tac_toe: Option<TicTacToe>,
    kiosk_mode: Option<KioskMode>,
    kiosk_mode_enabled: bool,
    color_palette: ColorPalette,
}

impl WordsWithToddlers {
    /// Creates a new instance of the application
    pub fn new() -> (Self, Task<Message>) {
        // Load saved configuration
        let config = crate::config::load_config();

        (
            WordsWithToddlers {
                letters: Vec::new(),
                discovered_words: Vec::new(),
                dictionary: Dictionary::new(),
                is_fullscreen: false,
                letters_scroll_id: ScrollableId::unique(),
                has_started_typing: false,
                cursor_visible: true,
                sound_playing: Arc::new(AtomicBool::new(false)),
                current_screen: Screen::Settings,
                selected_sound: config.selected_sound,
                use_uppercase: config.use_uppercase,
                word_list_loader: WordListLoader::new(),
                word_challenge: None,
                celebration: None,
                tic_tac_toe: None,
                kiosk_mode: None,
                kiosk_mode_enabled: config.kiosk_mode_enabled,
                color_palette: config.color_palette,
            },
            // Send a message after a short delay to set window to AlwaysOnTop
            Task::perform(
                async {
                    // Small delay to ensure window is created in current Space
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                },
                |_| Message::WindowOpened,
            ),
        )
    }

    /// Handles all application messages and updates state accordingly
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::KeyPressed(key) => self.handle_key_press(key),
            Message::ToggleFullscreen => self.toggle_fullscreen(),
            Message::WindowOpened => {
                // Set window to AlwaysOnTop after it's been created on current display
                window::change_level(window::Id::unique(), window::Level::AlwaysOnTop)
            }
            Message::ToggleCursor => {
                self.cursor_visible = !self.cursor_visible;
                Task::none()
            }
            Message::NavigateToSettings => {
                self.current_screen = Screen::Settings;
                Task::none()
            }
            Message::NavigateToWelcome => {
                self.current_screen = Screen::Welcome;
                self.word_challenge = None;
                self.celebration = None;
                Task::none()
            }
            Message::NavigateToMain => {
                self.current_screen = Screen::Main;
                Task::none()
            }
            Message::StartVisualChallenge => {
                if let Some(words) = self.word_list_loader.get_words_for_grade(GradeLevel::PreK) {
                    self.word_challenge =
                        Some(WordChallenge::new(ChallengeMode::Visual, words.clone()));
                    self.current_screen = Screen::WordChallenge;

                    // Speak the first word
                    if let Some(ref challenge) = self.word_challenge {
                        crate::speech::speak_word_async(challenge.current_word.clone());
                    }
                }
                Task::none()
            }
            Message::StartAudioChallenge => {
                if let Some(words) = self.word_list_loader.get_words_for_grade(GradeLevel::PreK) {
                    self.word_challenge =
                        Some(WordChallenge::new(ChallengeMode::Audio, words.clone()));
                    self.current_screen = Screen::WordChallenge;

                    // Speak the first word
                    if let Some(ref challenge) = self.word_challenge {
                        crate::speech::speak_word_async(challenge.current_word.clone());
                    }
                }
                Task::none()
            }
            Message::ReplayWord => {
                if let Some(ref challenge) = self.word_challenge {
                    crate::speech::speak_word_async(challenge.current_word.clone());
                }
                Task::none()
            }
            Message::CheckTypedWord => {
                if let Some(ref mut challenge) = self.word_challenge {
                    if challenge.check_if_correct() {
                        challenge.handle_correct_word();
                        self.celebration = Some(Celebration::new());

                        // Play success sound with fresh flag
                        self.sound_playing
                            .store(false, std::sync::atomic::Ordering::Relaxed);
                        let sound_path = crate::system_sound::get_sound_path(&self.selected_sound);
                        crate::audio::play_sound(
                            self.sound_playing.clone(),
                            sound_path.to_string(),
                        );

                        // Schedule celebration finish
                        return Task::perform(
                            async {
                                tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                            },
                            |_| Message::FinishCelebration,
                        );
                    } else {
                        challenge.handle_incorrect_word();
                    }
                }
                Task::none()
            }
            Message::FinishCelebration => {
                if let Some(ref mut challenge) = self.word_challenge {
                    // Check if we should level up or down
                    if challenge.should_level_up() {
                        if let Some(new_level) = challenge.level_up() {
                            if let Some(words) =
                                self.word_list_loader.get_words_for_grade(new_level)
                            {
                                challenge.update_word_list(words.clone());
                            }
                        }
                        // Still need to clear celebration state
                        challenge.is_celebrating = false;
                    } else if challenge.should_level_down() {
                        if let Some(new_level) = challenge.level_down() {
                            if let Some(words) =
                                self.word_list_loader.get_words_for_grade(new_level)
                            {
                                challenge.update_word_list(words.clone());
                            }
                        }
                        // Still need to clear celebration state
                        challenge.is_celebrating = false;
                    } else {
                        challenge.finish_celebration();
                    }

                    // Speak the new word for both visual and audio modes
                    crate::speech::speak_word_async(challenge.current_word.clone());
                }
                self.celebration = None;
                Task::none()
            }
            Message::ExitChallenge => {
                // Save challenge session
                if let Some(ref challenge) = self.word_challenge {
                    let session = Session::new_challenge(
                        challenge.grade_level,
                        challenge.score,
                        challenge.words_completed,
                    );
                    if let Err(e) = session.save() {
                        eprintln!("Failed to save challenge session: {}", e);
                    }
                }
                self.word_challenge = None;
                self.celebration = None;
                self.current_screen = Screen::Welcome;
                Task::none()
            }
            Message::SelectSound(sound_name) => {
                self.selected_sound = sound_name.clone();

                // Play the newly selected sound
                let sound_path = crate::system_sound::get_sound_path(&sound_name);
                crate::audio::play_sound(self.sound_playing.clone(), sound_path.to_string());

                // Save configuration
                let config = crate::config::AppConfig {
                    selected_sound: sound_name,
                    last_selected_grade: GradeLevel::default(),
                    use_uppercase: self.use_uppercase,
                    kiosk_mode_enabled: self.kiosk_mode_enabled,
                    color_palette: self.color_palette.clone(),
                };
                if let Err(e) = crate::config::save_config(&config) {
                    eprintln!("Failed to save config: {}", e);
                }
                Task::none()
            }

            Message::ToggleUppercase(value) => {
                self.use_uppercase = value;

                // Save configuration
                let config = crate::config::AppConfig {
                    selected_sound: self.selected_sound.clone(),
                    last_selected_grade: GradeLevel::default(),
                    use_uppercase: value,
                    kiosk_mode_enabled: self.kiosk_mode_enabled,
                    color_palette: self.color_palette.clone(),
                };
                if let Err(e) = crate::config::save_config(&config) {
                    eprintln!("Failed to save config: {}", e);
                }
                Task::none()
            }

            Message::StartTicTacToe => {
                self.tic_tac_toe = Some(TicTacToe::new(crate::tic_tac_toe::GameMode::TwoPlayer));
                self.current_screen = Screen::TicTacToe;
                Task::none()
            }
            Message::StartTicTacToeOnePlayer => {
                self.tic_tac_toe = Some(TicTacToe::new(crate::tic_tac_toe::GameMode::OnePlayer));
                self.current_screen = Screen::TicTacToe;
                Task::none()
            }
            Message::StartTicTacToeTwoPlayer => {
                self.tic_tac_toe = Some(TicTacToe::new(crate::tic_tac_toe::GameMode::TwoPlayer));
                self.current_screen = Screen::TicTacToe;
                Task::none()
            }
            Message::TicTacToeMove(position) => {
                if let Some(ref mut game) = self.tic_tac_toe {
                    game.make_move(position);
                    if game.is_computer_turn() {
                        return Task::done(Message::TicTacToeComputerMove);
                    }
                }
                Task::none()
            }
            Message::TicTacToeComputerMove => {
                if let Some(ref mut game) = self.tic_tac_toe {
                    game.computer_move();
                }
                Task::none()
            }
            Message::ResetTicTacToe => {
                if let Some(ref mut game) = self.tic_tac_toe {
                    game.reset();
                }
                Task::none()
            }
            Message::ExitTicTacToe => {
                self.tic_tac_toe = None;
                self.current_screen = Screen::Welcome;
                Task::none()
            }
            Message::ToggleKioskMode(enable) => {
                #[cfg(target_os = "macos")]
                {
                    if enable {
                        let has_permission = crate::kiosk_mode::permission::has_accessibility_permission();
                        eprintln!("Kiosk mode requested. Has accessibility permission: {}", has_permission);

                        if !has_permission {
                            eprintln!("Requesting accessibility permission...");
                            return Task::done(Message::RequestAccessibilityPermission);
                        }

                        eprintln!("Starting kiosk mode...");
                        match KioskMode::start() {
                            Ok(kiosk) => {
                                self.kiosk_mode = Some(kiosk);
                                self.kiosk_mode_enabled = true;
                                self.save_kiosk_config(true);
                            }
                            Err(e) => {
                                eprintln!("Failed to start kiosk mode: {}", e);
                                return Task::done(Message::KioskModeStatusChanged(
                                    KioskModeStatus::Error(e.to_string()),
                                ));
                            }
                        }
                    } else {
                        if let Some(mut kiosk) = self.kiosk_mode.take() {
                            kiosk.stop();
                        }
                        self.kiosk_mode_enabled = false;
                        self.save_kiosk_config(false);
                    }
                }

                #[cfg(not(target_os = "macos"))]
                {
                    let _ = enable;
                    eprintln!("Kiosk mode is only available on macOS");
                }

                Task::none()
            }
            Message::KioskModeStatusChanged(status) => {
                match status {
                    KioskModeStatus::Error(msg) => {
                        eprintln!("Kiosk mode error: {}", msg);
                    }
                    KioskModeStatus::PermissionRequired => {
                        eprintln!("Accessibility permission required for kiosk mode");
                    }
                    _ => {}
                }
                Task::none()
            }
            Message::RequestAccessibilityPermission => {
                #[cfg(target_os = "macos")]
                {
                    let granted = crate::kiosk_mode::permission::request_accessibility_permission();
                    if !granted {
                        crate::kiosk_mode::permission::open_accessibility_preferences();
                    }
                }
                Task::none()
            }
            Message::SelectColorPalette(palette) => {
                self.color_palette = palette.clone();

                let config = crate::config::AppConfig {
                    selected_sound: self.selected_sound.clone(),
                    last_selected_grade: GradeLevel::default(),
                    use_uppercase: self.use_uppercase,
                    kiosk_mode_enabled: self.kiosk_mode_enabled,
                    color_palette: palette,
                };
                if let Err(e) = crate::config::save_config(&config) {
                    eprintln!("Failed to save config: {}", e);
                }
                Task::none()
            }
        }
    }

    /// Saves kiosk mode setting to config
    fn save_kiosk_config(&self, enabled: bool) {
        let config = crate::config::AppConfig {
            selected_sound: self.selected_sound.clone(),
            last_selected_grade: GradeLevel::default(),
            use_uppercase: self.use_uppercase,
            kiosk_mode_enabled: enabled,
            color_palette: self.color_palette.clone(),
        };
        if let Err(e) = crate::config::save_config(&config) {
            eprintln!("Failed to save kiosk config: {}", e);
        }
    }

    /// Builds the user interface
    pub fn view(&self) -> Element<'_, Message> {
        // Route to different screens based on current_screen
        match self.current_screen {
            Screen::Welcome => self.build_welcome_screen(),
            Screen::Settings => self.build_settings_screen(),
            Screen::WordChallenge => self.build_word_challenge_screen(),
            Screen::TicTacToe => self.build_tic_tac_toe_screen(),
            Screen::Main => {
                let mut main_column = column![].spacing(20).align_x(alignment::Horizontal::Center);

                // Add discovered words at the top if any exist
                if !self.discovered_words.is_empty() {
                    main_column = main_column.push(self.build_discovered_words_display());
                }

                // Add the main content
                let content = if self.letters.is_empty() {
                    if self.has_started_typing {
                        self.build_placeholder_screen()
                    } else {
                        self.build_welcome_screen()
                    }
                } else {
                    self.build_letters_display()
                };

                main_column = main_column.push(content);

                container(main_column)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(20)
                    .style(|_theme: &Theme| container::Style {
                        background: Some(iced::Background::Color(Color::from_rgb(0.05, 0.05, 0.1))),
                        ..Default::default()
                    })
                    .into()
            }
        }
    }

    /// Sets up event subscriptions for keyboard input and cursor blinking
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            event::listen_with(|event, _status, _id| match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                    Some(Message::KeyPressed(key))
                }
                _ => None,
            }),
            iced::time::every(std::time::Duration::from_millis(530)).map(|_| Message::ToggleCursor),
        ])
    }

    /// Returns the application theme
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    /// Handles keyboard input
    fn handle_key_press(&mut self, key: keyboard::Key) -> Task<Message> {
        // Handle challenge mode separately
        if self.current_screen == Screen::WordChallenge {
            return self.handle_challenge_key_press(key);
        }

        // Handle tic-tac-toe separately
        if self.current_screen == Screen::TicTacToe {
            return self.handle_tictactoe_key_press(key);
        }

        // Handle Settings screen - Escape goes back to Welcome
        if self.current_screen == Screen::Settings {
            if let keyboard::Key::Named(keyboard::key::Named::Escape) = key {
                self.current_screen = Screen::Welcome;
                return Task::none();
            }
            return Task::none();
        }

        match key {
            keyboard::Key::Named(keyboard::key::Named::Escape) => {
                return exit();
            }
            keyboard::Key::Named(keyboard::key::Named::Enter) => {
                // Save session if we have typed anything
                if !self.letters.is_empty() {
                    let typed_text: String = self.letters.iter().map(|l| l.character).collect();

                    let session = Session::new(
                        typed_text,
                        self.discovered_words
                            .iter()
                            .map(|w| w.text.clone())
                            .collect(),
                    );

                    // Try to save the session
                    match session.save() {
                        Ok(path) => eprintln!("Session saved to: {:?}", path),
                        Err(e) => eprintln!("Failed to save session: {}", e),
                    }
                }

                // Check for any last word before clearing
                self.check_and_save_word();

                // Play selected sound effect
                let sound_path = crate::system_sound::get_sound_path(&self.selected_sound);
                crate::audio::play_sound(self.sound_playing.clone(), sound_path.to_string());

                // Clear letters and discovered words, mark that we've started typing
                self.letters.clear();
                self.discovered_words.clear();
                self.has_started_typing = true;
                // Return to welcome screen
                self.current_screen = Screen::Welcome;
            }
            keyboard::Key::Named(keyboard::key::Named::F11) => {
                return Task::done(Message::ToggleFullscreen);
            }
            keyboard::Key::Named(keyboard::key::Named::Backspace) => {
                self.letters.pop();
            }
            keyboard::Key::Character(s) => {
                self.add_character_from_string(s.to_string());
                return scrollable::snap_to(
                    self.letters_scroll_id.clone(),
                    scrollable::RelativeOffset { x: 0.0, y: 1.0 },
                );
            }
            keyboard::Key::Named(keyboard::key::Named::Space) => {
                // Check for word before adding space
                self.check_and_save_word();
                self.add_space();
                return scrollable::snap_to(
                    self.letters_scroll_id.clone(),
                    scrollable::RelativeOffset { x: 0.0, y: 1.0 },
                );
            }
            _ => {}
        }
        Task::none()
    }

    /// Adds a character from the typed string
    fn add_character_from_string(&mut self, s: String) {
        if let Some(c) = s.chars().next() {
            if c.is_alphabetic() || c.is_numeric() {
                // Switch to Main screen on first character
                if self.current_screen == Screen::Welcome {
                    self.current_screen = Screen::Main;
                }

                let character = if c.is_alphabetic() {
                    c.to_uppercase().next().unwrap()
                } else {
                    c
                };
                self.letters
                    .push(Letter::new(character, self.random_color()));
            }
        }
    }

    /// Adds a space character
    fn add_space(&mut self) {
        self.letters.push(Letter::new(' ', self.random_color()));
    }

    /// Checks the last word segment (since last space) and finds all valid words
    fn check_and_save_word(&mut self) {
        // Find the last space position (or start from beginning)
        let last_space_pos = self
            .letters
            .iter()
            .rposition(|l| l.character == ' ')
            .map(|pos| pos + 1)
            .unwrap_or(0);

        // Get only the letters since the last space
        let last_segment: String = self.letters[last_space_pos..]
            .iter()
            .map(|l| l.character)
            .collect();

        // Skip if empty or only spaces
        if last_segment.trim().is_empty() {
            return;
        }

        // Parse compound words from the segment
        let found_words = self.dictionary.parse_compound_words(&last_segment);

        // Add each found word (duplicates are OK)
        for word in found_words {
            let color = self.random_color();
            self.discovered_words.push(DiscoveredWord::new(word, color));

            // Keep only the last 20 words
            if self.discovered_words.len() > 20 {
                self.discovered_words.remove(0);
            }
        }
    }

    /// Toggles fullscreen mode
    fn toggle_fullscreen(&mut self) -> Task<Message> {
        self.is_fullscreen = !self.is_fullscreen;
        window::change_mode(
            window::Id::unique(),
            if self.is_fullscreen {
                window::Mode::Fullscreen
            } else {
                window::Mode::Windowed
            },
        )
    }

    /// Builds the placeholder screen shown after typing has started
    fn build_placeholder_screen(&self) -> Element<'_, Message> {
        let placeholder_text = text("Ready for more words...")
            .size(60)
            .color(Color::from_rgb(0.4, 0.4, 0.45));

        container(placeholder_text)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    /// Builds the welcome screen with multicolored title
    fn build_welcome_screen(&self) -> Element<'_, Message> {
        let welcome_text = "Welcome to Words With Toddlers!";
        let mut welcome_row = row![].spacing(5).align_y(alignment::Vertical::Center);

        for (i, ch) in welcome_text.chars().enumerate() {
            let hue = (i as f32 * 360.0 / welcome_text.len() as f32) % 360.0;
            let (r, g, b) = hsl_to_rgb(hue, 0.8, 0.6);
            welcome_row = welcome_row.push(
                text(ch.to_string())
                    .size(50)
                    .color(Color::from_rgb(r, g, b)),
            );
        }

        let instructions = text("Type any letter to start Discovery Mode!\nPress Space to save words • Enter to clear all • Escape to exit")
            .size(30)
            .color(Color::from_rgb(0.6, 0.6, 0.6));

        // Visual challenge button
        let visual_button = button(text("👁️ See Words").size(30))
            .padding(20)
            .style(|_theme: &Theme, _status| button::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.2, 0.6, 0.9))),
                border: iced::Border {
                    color: Color::from_rgb(0.4, 0.8, 1.0),
                    width: 2.0,
                    radius: 10.0.into(),
                },
                ..Default::default()
            })
            .on_press(Message::StartVisualChallenge);

        // Audio challenge button
        let audio_button = button(text("🔊 Hear Words").size(30))
            .padding(20)
            .style(|_theme: &Theme, _status| button::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.6, 0.2, 0.9))),
                border: iced::Border {
                    color: Color::from_rgb(0.8, 0.4, 1.0),
                    width: 2.0,
                    radius: 10.0.into(),
                },
                ..Default::default()
            })
            .on_press(Message::StartAudioChallenge);

        let challenge_row = row![visual_button, audio_button]
            .spacing(20)
            .align_y(alignment::Vertical::Center);

        // Tic Tac Toe button
        let tictactoe_button = button(text("❌⭕ Tic Tac Toe").size(30))
            .padding(20)
            .style(|_theme: &Theme, _status| button::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.9, 0.5, 0.2))),
                border: iced::Border {
                    color: Color::from_rgb(1.0, 0.7, 0.4),
                    width: 2.0,
                    radius: 10.0.into(),
                },
                ..Default::default()
            })
            .on_press(Message::StartTicTacToe);

        // Settings button
        let settings_button = button(text("⚙️  Settings").size(25))
            .padding(15)
            .style(|_theme: &Theme, _status| button::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.25, 0.25, 0.3))),
                border: iced::Border {
                    color: Color::from_rgb(0.4, 0.4, 0.45),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                ..Default::default()
            })
            .on_press(Message::NavigateToSettings);

        container(
            column![
                welcome_row,
                instructions,
                challenge_row,
                tictactoe_button,
                settings_button
            ]
            .spacing(30)
            .align_x(alignment::Horizontal::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
    }

    /// Builds the settings screen
    fn build_settings_screen(&self) -> Element<'_, Message> {
        let title = text("Settings")
            .size(48)
            .color(Color::from_rgb(0.9, 0.9, 1.0));

        // --- Left column: Sounds + Word Display ---

        let sound_label = text("Sound Effect")
            .size(28)
            .color(Color::from_rgb(0.9, 0.9, 1.0));

        let mut sounds_grid = column![].spacing(8).align_x(alignment::Horizontal::Center);
        let sounds = crate::system_sound::SOUNDS;
        for row_sounds in sounds.chunks(3) {
            let mut sound_row = row![].spacing(10).align_y(alignment::Vertical::Center);
            for sound in row_sounds {
                let is_selected = sound.name == self.selected_sound;
                let button_color = if is_selected {
                    Color::from_rgb(0.2, 0.6, 0.9)
                } else {
                    Color::from_rgb(0.3, 0.3, 0.35)
                };
                let sound_button =
                    button(text(sound.display_name).size(22).color(if is_selected {
                        Color::from_rgb(1.0, 1.0, 1.0)
                    } else {
                        Color::from_rgb(0.8, 0.8, 0.8)
                    }))
                    .padding(12)
                    .style(move |_theme: &Theme, _status| button::Style {
                        background: Some(iced::Background::Color(button_color)),
                        border: iced::Border {
                            color: if is_selected {
                                Color::from_rgb(0.4, 0.8, 1.0)
                            } else {
                                Color::from_rgb(0.4, 0.4, 0.45)
                            },
                            width: if is_selected { 3.0 } else { 1.0 },
                            radius: 8.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(Message::SelectSound(sound.name.to_string()));
                sound_row = sound_row.push(sound_button);
            }
            sounds_grid = sounds_grid.push(sound_row);
        }

        let case_label = text("Word Display")
            .size(28)
            .color(Color::from_rgb(0.9, 0.9, 1.0));

        let uppercase_button = button(text("ABC").size(22).color(
            if self.use_uppercase {
                Color::from_rgb(1.0, 1.0, 1.0)
            } else {
                Color::from_rgb(0.8, 0.8, 0.8)
            },
        ))
        .padding(12)
        .style(move |_theme: &Theme, _status| button::Style {
            background: Some(iced::Background::Color(if self.use_uppercase {
                Color::from_rgb(0.2, 0.6, 0.9)
            } else {
                Color::from_rgb(0.3, 0.3, 0.35)
            })),
            border: iced::Border {
                color: if self.use_uppercase {
                    Color::from_rgb(0.4, 0.8, 1.0)
                } else {
                    Color::from_rgb(0.4, 0.4, 0.45)
                },
                width: if self.use_uppercase { 3.0 } else { 1.0 },
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .on_press(Message::ToggleUppercase(true));

        let lowercase_button = button(text("abc").size(22).color(
            if !self.use_uppercase {
                Color::from_rgb(1.0, 1.0, 1.0)
            } else {
                Color::from_rgb(0.8, 0.8, 0.8)
            },
        ))
        .padding(12)
        .style(move |_theme: &Theme, _status| button::Style {
            background: Some(iced::Background::Color(if !self.use_uppercase {
                Color::from_rgb(0.2, 0.6, 0.9)
            } else {
                Color::from_rgb(0.3, 0.3, 0.35)
            })),
            border: iced::Border {
                color: if !self.use_uppercase {
                    Color::from_rgb(0.4, 0.8, 1.0)
                } else {
                    Color::from_rgb(0.4, 0.4, 0.45)
                },
                width: if !self.use_uppercase { 3.0 } else { 1.0 },
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .on_press(Message::ToggleUppercase(false));

        let case_toggle_row = row![uppercase_button, lowercase_button]
            .spacing(10)
            .align_y(alignment::Vertical::Center);

        // Kiosk mode section (macOS only)
        #[cfg(target_os = "macos")]
        let kiosk_section = {
            let kiosk_label = text("Kiosk Mode")
                .size(28)
                .color(Color::from_rgb(0.9, 0.9, 1.0));

            let kiosk_desc = text("Blocks Cmd+Tab and similar shortcuts")
                .size(16)
                .color(Color::from_rgb(0.6, 0.6, 0.6));

            let kiosk_enabled = self.kiosk_mode_enabled;

            let enable_button = button(text("On").size(22).color(
                if kiosk_enabled {
                    Color::from_rgb(1.0, 1.0, 1.0)
                } else {
                    Color::from_rgb(0.8, 0.8, 0.8)
                },
            ))
            .padding(12)
            .style(move |_theme: &Theme, _status| button::Style {
                background: Some(iced::Background::Color(if kiosk_enabled {
                    Color::from_rgb(0.2, 0.7, 0.3)
                } else {
                    Color::from_rgb(0.3, 0.3, 0.35)
                })),
                border: iced::Border {
                    color: if kiosk_enabled {
                        Color::from_rgb(0.4, 0.9, 0.5)
                    } else {
                        Color::from_rgb(0.4, 0.4, 0.45)
                    },
                    width: if kiosk_enabled { 3.0 } else { 1.0 },
                    radius: 8.0.into(),
                },
                ..Default::default()
            })
            .on_press(Message::ToggleKioskMode(true));

            let disable_button = button(text("Off").size(22).color(
                if !kiosk_enabled {
                    Color::from_rgb(1.0, 1.0, 1.0)
                } else {
                    Color::from_rgb(0.8, 0.8, 0.8)
                },
            ))
            .padding(12)
            .style(move |_theme: &Theme, _status| button::Style {
                background: Some(iced::Background::Color(if !kiosk_enabled {
                    Color::from_rgb(0.6, 0.2, 0.2)
                } else {
                    Color::from_rgb(0.3, 0.3, 0.35)
                })),
                border: iced::Border {
                    color: if !kiosk_enabled {
                        Color::from_rgb(0.8, 0.4, 0.4)
                    } else {
                        Color::from_rgb(0.4, 0.4, 0.45)
                    },
                    width: if !kiosk_enabled { 3.0 } else { 1.0 },
                    radius: 8.0.into(),
                },
                ..Default::default()
            })
            .on_press(Message::ToggleKioskMode(false));

            let kiosk_toggle_row = row![enable_button, disable_button]
                .spacing(10)
                .align_y(alignment::Vertical::Center);

            column![kiosk_label, kiosk_desc, kiosk_toggle_row]
                .spacing(10)
                .align_x(alignment::Horizontal::Center)
        };

        #[cfg(not(target_os = "macos"))]
        let kiosk_section = column![];

        let left_column = column![sound_label, sounds_grid, case_label, case_toggle_row, kiosk_section]
            .spacing(20)
            .align_x(alignment::Horizontal::Center)
            .width(Length::FillPortion(1));

        // --- Right column: Color Palette ---

        let palette_label = text("Letter Colors")
            .size(28)
            .color(Color::from_rgb(0.9, 0.9, 1.0));

        let mut palette_grid = column![].spacing(8).align_x(alignment::Horizontal::Center);
        let palettes = ColorPalette::all();
        for row_palettes in palettes.chunks(2) {
            let mut palette_row = row![].spacing(10).align_y(alignment::Vertical::Center);
            for palette in row_palettes {
                let is_selected = *palette == self.color_palette;
                let (pr, pg, pb) = palette.preview_color();
                let preview_color = Color::from_rgb(pr, pg, pb);
                let palette_clone = palette.clone();

                let label_row = row![
                    text("\u{25CF} ").size(20).color(preview_color),
                    text(palette.display_name()).size(20).color(if is_selected {
                        Color::from_rgb(1.0, 1.0, 1.0)
                    } else {
                        Color::from_rgb(0.8, 0.8, 0.8)
                    }),
                ]
                .align_y(alignment::Vertical::Center);

                let button_color = if is_selected {
                    Color::from_rgb(0.2, 0.6, 0.9)
                } else {
                    Color::from_rgb(0.3, 0.3, 0.35)
                };

                let palette_button = button(label_row)
                    .padding(12)
                    .width(Length::Fill)
                    .style(move |_theme: &Theme, _status| button::Style {
                        background: Some(iced::Background::Color(button_color)),
                        border: iced::Border {
                            color: if is_selected {
                                Color::from_rgb(0.4, 0.8, 1.0)
                            } else {
                                Color::from_rgb(0.4, 0.4, 0.45)
                            },
                            width: if is_selected { 3.0 } else { 1.0 },
                            radius: 8.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(Message::SelectColorPalette(palette_clone));

                palette_row = palette_row.push(palette_button);
            }
            palette_grid = palette_grid.push(palette_row);
        }

        let right_column = column![palette_label, palette_grid]
            .spacing(20)
            .align_x(alignment::Horizontal::Center)
            .width(Length::FillPortion(1));

        // --- Assemble layout ---

        let columns = row![left_column, right_column]
            .spacing(40)
            .align_y(alignment::Vertical::Top);

        let back_button = button(text("\u{2190} Back to Welcome").size(22))
            .padding(12)
            .on_press(Message::NavigateToWelcome);

        let content = column![title, columns, back_button]
            .spacing(25)
            .padding(30)
            .align_x(alignment::Horizontal::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    /// Builds the display for discovered words with wrapping
    fn build_discovered_words_display(&self) -> Element<'_, Message> {
        // Create a column to hold multiple rows of words
        let mut words_column = column![].spacing(10).align_x(alignment::Horizontal::Center);

        // Build rows of words, wrapping when needed
        let mut current_row: Vec<Element<Message>> = Vec::new();
        let mut row_width = 0.0;
        let max_width = 1100.0; // Approximate max width before wrapping
        let char_width = 25.0; // Approximate width per character at size 40

        for word in &self.discovered_words {
            let word_width = word.text.len() as f32 * char_width + 20.0; // Add spacing

            // Check if we need to wrap to next line
            if row_width + word_width > max_width && !current_row.is_empty() {
                // Add current row to column
                let mut row_container = Row::new().spacing(20).align_y(alignment::Vertical::Center);
                for elem in current_row.drain(..) {
                    row_container = row_container.push(elem);
                }
                words_column = words_column.push(row_container);
                row_width = 0.0;
            }

            // Add word to current row
            current_row.push(text(&word.text).size(40).color(word.color).into());
            row_width += word_width;
        }

        // Add any remaining words in the last row
        if !current_row.is_empty() {
            let mut row_container = Row::new().spacing(20).align_y(alignment::Vertical::Center);
            for elem in current_row {
                row_container = row_container.push(elem);
            }
            words_column = words_column.push(row_container);
        }

        container(
            scrollable(words_column).direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::default(),
            )),
        )
        .width(Length::Fill)
        .height(Length::Shrink)
        .max_height(150) // Limit height to prevent taking too much space
        .padding(10)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(
                0.1, 0.1, 0.15, 0.5,
            ))),
            border: iced::Border {
                color: Color::from_rgba(0.3, 0.3, 0.4, 0.3),
                width: 1.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        })
        .into()
    }

    /// Builds the display for typed letters with word wrapping
    fn build_letters_display(&self) -> Element<'_, Message> {
        // Fixed letter size for consistency
        let letter_size = 120;

        // Create a column to hold multiple rows of letters
        let mut letters_column = column![].spacing(10).align_x(alignment::Horizontal::Center);

        // Calculate how many letters fit per row based on window width
        let letters_per_row = 15; // Approximate for 1200px width with 120px letters

        // Build rows of letters
        let mut current_row = row![].spacing(5).align_y(alignment::Vertical::Center);
        let mut letter_count = 0;

        for letter in &self.letters {
            // Add letter to current row
            current_row = current_row.push(
                text(letter.character.to_string())
                    .size(letter_size)
                    .color(letter.color),
            );
            letter_count += 1;

            // Check if we need to wrap to next line
            if letter_count >= letters_per_row {
                letters_column = letters_column.push(current_row);
                current_row = row![].spacing(5).align_y(alignment::Vertical::Center);
                letter_count = 0;
            }
        }

        // Add blinking cursor after the last letter
        // Always add cursor to prevent layout shift, but toggle transparency
        let cursor_color = if self.cursor_visible {
            Color::from_rgb(1.0, 1.0, 1.0)
        } else {
            Color::from_rgba(1.0, 1.0, 1.0, 0.0)
        };
        current_row = current_row.push(text("|").size(letter_size).color(cursor_color));

        // Add any remaining letters (and cursor) in the last row
        // Cursor is always present now, so always add the row
        letters_column = letters_column.push(current_row);

        // Wrap in scrollable container if content gets too tall
        container(
            scrollable(letters_column)
                .id(self.letters_scroll_id.clone())
                .direction(scrollable::Direction::Vertical(
                    scrollable::Scrollbar::default(),
                )),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
    }

    /// Generates a random color based on the selected palette
    fn random_color(&self) -> Color {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let (hue_min, hue_max, saturation, lightness) = self.color_palette.color_params();
        let hue = rng.gen_range(hue_min..hue_max);
        let (r, g, b) = hsl_to_rgb(hue, saturation, lightness);
        Color::from_rgb(r, g, b)
    }

    /// Builds the word challenge screen
    fn build_word_challenge_screen(&self) -> Element<'_, Message> {
        if let Some(ref challenge) = self.word_challenge {
            let mut content_column = column![].spacing(40).align_x(alignment::Horizontal::Center);

            // Score and difficulty display
            let header_row = row![
                text(format!("Score: {}", challenge.score))
                    .size(40)
                    .color(Color::from_rgb(1.0, 0.8, 0.2)),
                text(format!("Level: {}", challenge.grade_level.display_name()))
                    .size(40)
                    .color(Color::from_rgb(0.5, 1.0, 0.8)),
            ]
            .spacing(60)
            .align_y(alignment::Vertical::Center);

            content_column = content_column.push(header_row);

            // Target word display
            let target_word_color = if challenge.is_celebrating {
                Color::from_rgb(0.2, 1.0, 0.3)
            } else {
                Color::from_rgb(0.9, 0.9, 1.0)
            };

            let target_size = if challenge.is_celebrating {
                if let Some(ref celebration) = self.celebration {
                    (150.0 * celebration.scale_factor()) as u16
                } else {
                    150
                }
            } else {
                150
            };

            let displayed_word = if self.use_uppercase {
                challenge.current_word.to_uppercase()
            } else {
                challenge.current_word.clone()
            };
            let target_word = text(displayed_word)
                .size(target_size)
                .color(target_word_color);

            // Show target word in visual mode OR in audio mode after 3 wrong attempts
            if challenge.mode == ChallengeMode::Visual || challenge.should_reveal_word() {
                if challenge.should_reveal_word() {
                    // Show hint text in audio mode when revealing
                    let hint_text = text("Here's the word to help you:")
                        .size(30)
                        .color(Color::from_rgb(1.0, 0.7, 0.3));
                    content_column = content_column.push(hint_text);
                }
                content_column = content_column.push(target_word);
            } else {
                // In audio mode, show replay button
                let replay_button = button(text("🔊 Replay Word").size(50))
                    .padding(30)
                    .style(|_theme: &Theme, _status| button::Style {
                        background: Some(iced::Background::Color(Color::from_rgb(0.6, 0.2, 0.9))),
                        border: iced::Border {
                            color: Color::from_rgb(0.8, 0.4, 1.0),
                            width: 2.0,
                            radius: 15.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(Message::ReplayWord);

                content_column = content_column.push(replay_button);
            }

            // Typed letters display
            if !challenge.is_celebrating {
                let mut typed_row = row![].spacing(5).align_y(alignment::Vertical::Center);

                for letter in &challenge.typed_letters {
                    typed_row = typed_row.push(
                        text(letter.character.to_string())
                            .size(100)
                            .color(letter.color),
                    );
                }

                // Add cursor
                let cursor_color = if self.cursor_visible {
                    Color::from_rgb(1.0, 1.0, 1.0)
                } else {
                    Color::from_rgba(1.0, 1.0, 1.0, 0.0)
                };
                typed_row = typed_row.push(text("|").size(100).color(cursor_color));

                content_column = content_column.push(typed_row);
            } else if let Some(ref celebration) = self.celebration {
                let celebration_text = text("✓ Correct!").size(80).color(Color::from_rgba(
                    0.2,
                    1.0,
                    0.3,
                    celebration.opacity(),
                ));

                content_column = content_column.push(celebration_text);
            }

            // Instructions
            let instructions = if challenge.mode == ChallengeMode::Visual {
                text("Type the word shown above\nPress ESC to exit")
                    .size(25)
                    .color(Color::from_rgb(0.5, 0.5, 0.6))
            } else {
                text("Type the word you hear\nPress 🔊 to replay • Press ESC to exit")
                    .size(25)
                    .color(Color::from_rgb(0.5, 0.5, 0.6))
            };

            content_column = content_column.push(instructions);

            container(content_column)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .style(|_theme: &Theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.05, 0.05, 0.1))),
                    ..Default::default()
                })
                .into()
        } else {
            // Fallback if no challenge exists
            self.build_welcome_screen()
        }
    }

    /// Builds the tic-tac-toe game screen
    fn build_tic_tac_toe_screen(&self) -> Element<'_, Message> {
        if let Some(ref game) = self.tic_tac_toe {
            let title = text("Tic Tac Toe")
                .size(60)
                .color(Color::from_rgb(0.9, 0.9, 1.0));

            // Game status message with special styling
            let status_element: Element<'_, Message> = match &game.game_state {
                crate::tic_tac_toe::GameState::Playing => {
                    let turn_text = if game.mode == crate::tic_tac_toe::GameMode::OnePlayer {
                        if game.current_player() == crate::tic_tac_toe::Player::X {
                            "Your Turn (X)".to_string()
                        } else {
                            "Computer's Turn (O)".to_string()
                        }
                    } else {
                        format!("Player {}'s Turn", game.current_player())
                    };
                    text(turn_text)
                        .size(40)
                        .color(Color::from_rgb(0.8, 0.8, 0.9))
                        .into()
                }
                crate::tic_tac_toe::GameState::Won(player) => {
                    let win_text = if game.mode == crate::tic_tac_toe::GameMode::OnePlayer {
                        if *player == crate::tic_tac_toe::Player::X {
                            "You Win!".to_string()
                        } else {
                            "Computer Wins!".to_string()
                        }
                    } else {
                        format!("Player {} Wins!", player)
                    };
                    let mut rainbow_row = row![].spacing(2).align_y(alignment::Vertical::Center);

                    for (i, ch) in win_text.chars().enumerate() {
                        let hue = (i as f32 * 30.0) % 360.0;
                        let (r, g, b) = hsl_to_rgb(hue, 0.8, 0.6);
                        let char_text = text(ch.to_string())
                            .size(40)
                            .color(Color::from_rgb(r, g, b));
                        rainbow_row = rainbow_row.push(char_text);
                    }

                    rainbow_row.into()
                }
                crate::tic_tac_toe::GameState::Draw => {
                    // CATS game with cat emojis
                    text("🐱 CATS 🐱 GAME 🐱")
                        .size(40)
                        .color(Color::from_rgb(0.9, 0.7, 0.4))
                        .into()
                }
            };

            // Build the 3x3 grid with winning line highlight
            let mut board_rows = column![].spacing(10).align_x(alignment::Horizontal::Center);

            // Check if each position is part of the winning line
            let is_winning_cell = |pos: usize| -> bool {
                if let Some(line) = game.winning_line {
                    line.contains(&pos)
                } else {
                    false
                }
            };

            for row in 0..3 {
                let mut board_row = row![].spacing(10).align_y(alignment::Vertical::Center);

                for col in 0..3 {
                    let position = row * 3 + col;
                    let cell_content = match game.get_cell(position) {
                        Some(crate::tic_tac_toe::Player::X) => "X".to_string(),
                        Some(crate::tic_tac_toe::Player::O) => "O".to_string(),
                        None => format!("{}", position + 1),
                    };

                    let cell_color = match game.get_cell(position) {
                        Some(crate::tic_tac_toe::Player::X) => Color::from_rgb(0.3, 0.7, 1.0),
                        Some(crate::tic_tac_toe::Player::O) => Color::from_rgb(1.0, 0.5, 0.3),
                        None => Color::from_rgb(0.5, 0.5, 0.5),
                    };

                    // Highlight winning cells with rainbow border
                    let is_winner = is_winning_cell(position);
                    let border_color = if is_winner {
                        // Rainbow effect for winning line
                        let hue = (position as f32 * 120.0) % 360.0;
                        let (r, g, b) = hsl_to_rgb(hue, 1.0, 0.5);
                        Color::from_rgb(r, g, b)
                    } else {
                        Color::from_rgb(0.4, 0.4, 0.5)
                    };

                    let border_width = if is_winner { 5.0 } else { 3.0 };

                    let cell_label = container(text(cell_content).size(80).color(cell_color))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center);

                    let cell_button = button(cell_label)
                        .width(Length::Fixed(120.0))
                        .height(Length::Fixed(120.0))
                        .style(move |_theme: &Theme, _status| button::Style {
                            background: Some(iced::Background::Color(Color::from_rgb(
                                0.15, 0.15, 0.2,
                            ))),
                            border: iced::Border {
                                color: border_color,
                                width: border_width,
                                radius: 10.0.into(),
                            },
                            ..Default::default()
                        })
                        .on_press(Message::TicTacToeMove(position));

                    board_row = board_row.push(cell_button);
                }

                board_rows = board_rows.push(board_row);
            }

            // Control buttons
            let reset_button = button(text("🔄 New Game").size(25))
                .padding(15)
                .style(|_theme: &Theme, _status| button::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.3, 0.6, 0.3))),
                    border: iced::Border {
                        color: Color::from_rgb(0.5, 0.8, 0.5),
                        width: 2.0,
                        radius: 10.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(Message::ResetTicTacToe);

            let one_player_message = if game.mode == crate::tic_tac_toe::GameMode::OnePlayer {
                Message::StartTicTacToeTwoPlayer
            } else {
                Message::StartTicTacToeOnePlayer
            };
            let one_player_label = if game.mode == crate::tic_tac_toe::GameMode::OnePlayer {
                "🔄 New Game (2 Player)"
            } else {
                "🔄 New Game (1 Player)"
            };
            let switch_mode_button = button(text(one_player_label).size(25))
                .padding(15)
                .style(|_theme: &Theme, _status| button::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.2, 0.4, 0.7))),
                    border: iced::Border {
                        color: Color::from_rgb(0.3, 0.6, 0.9),
                        width: 2.0,
                        radius: 10.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(one_player_message);

            let back_button = button(text("⬅️  Back").size(25))
                .padding(15)
                .style(|_theme: &Theme, _status| button::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.25, 0.25, 0.3))),
                    border: iced::Border {
                        color: Color::from_rgb(0.4, 0.4, 0.45),
                        width: 1.0,
                        radius: 10.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(Message::ExitTicTacToe);

            let buttons_row = row![reset_button, switch_mode_button, back_button]
                .spacing(20)
                .align_y(alignment::Vertical::Center);

            let instructions = text("Click cells or press 1-9 to play • ESC to exit")
                .size(20)
                .color(Color::from_rgb(0.5, 0.5, 0.6));

            // Overlay winning line on board if there is one
            let board_widget: Element<'_, Message> = if let Some(winning_line) = game.winning_line {
                let line_overlay = WinningLineOverlay { winning_line };
                let line_canvas = canvas(line_overlay)
                    .width(Length::Fixed(380.0))
                    .height(Length::Fixed(380.0));

                stack![board_rows, line_canvas]
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .into()
            } else {
                board_rows.into()
            };

            container(
                column![title, status_element, board_widget, buttons_row, instructions]
                    .spacing(30)
                    .align_x(alignment::Horizontal::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
        } else {
            self.build_welcome_screen()
        }
    }

    /// Handles keyboard input in challenge mode
    fn handle_challenge_key_press(&mut self, key: keyboard::Key) -> Task<Message> {
        // Check if celebrating first
        if let Some(ref challenge) = self.word_challenge {
            if challenge.is_celebrating {
                return Task::none();
            }
        }

        match key {
            keyboard::Key::Named(keyboard::key::Named::Escape) => {
                return Task::done(Message::ExitChallenge);
            }
            keyboard::Key::Named(keyboard::key::Named::Backspace) => {
                if let Some(ref mut challenge) = self.word_challenge {
                    challenge.remove_last_letter();
                }
            }
            keyboard::Key::Named(keyboard::key::Named::Enter)
            | keyboard::Key::Named(keyboard::key::Named::Space) => {
                return Task::done(Message::CheckTypedWord);
            }
            keyboard::Key::Character(s) => {
                if let Some(c) = s.chars().next() {
                    if c.is_alphabetic() {
                        let character = c.to_uppercase().next().unwrap();
                        let color = self.random_color();

                        if let Some(ref mut challenge) = self.word_challenge {
                            challenge.add_letter(Letter::new(character, color));

                            // Auto-check if word length matches
                            if challenge.typed_text().len() == challenge.current_word.len() {
                                return Task::done(Message::CheckTypedWord);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        Task::none()
    }

    /// Handles keyboard input in tic-tac-toe mode
    fn handle_tictactoe_key_press(&mut self, key: keyboard::Key) -> Task<Message> {
        match key {
            keyboard::Key::Named(keyboard::key::Named::Escape) => {
                return Task::done(Message::ExitTicTacToe);
            }
            keyboard::Key::Character(s) => {
                if let Some(c) = s.chars().next() {
                    if c.is_numeric() {
                        if let Some(digit) = c.to_digit(10) {
                            // Convert 1-9 to 0-8 position
                            if (1..=9).contains(&digit) {
                                return Task::done(Message::TicTacToeMove((digit - 1) as usize));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        Task::none()
    }
}

/// Canvas program that draws a line through the winning cells in tic-tac-toe
struct WinningLineOverlay {
    winning_line: [usize; 3],
}

impl WinningLineOverlay {
    /// Converts a board position (0-8) to pixel coordinates within the board
    fn cell_center(position: usize) -> Point {
        let col = position % 3;
        let row = position / 3;
        Point::new(col as f32 * 130.0 + 60.0, row as f32 * 130.0 + 60.0)
    }
}

impl<Message> canvas::Program<Message> for WinningLineOverlay {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let start = Self::cell_center(self.winning_line[0]);
        let end = Self::cell_center(self.winning_line[2]);

        let line = canvas::Path::line(start, end);

        frame.stroke(
            &line,
            canvas::Stroke::default()
                .with_color(Color::from_rgba(1.0, 1.0, 0.2, 0.85))
                .with_width(8.0)
                .with_line_cap(canvas::stroke::LineCap::Round),
        );

        vec![frame.into_geometry()]
    }
}
