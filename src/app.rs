use iced::{
    alignment, event, keyboard, widget::{column, container, row, text, scrollable, Row, scrollable::Id as ScrollableId}, 
    window, Color, Element, Event, Length, Subscription, Task, Theme, exit
};
use crate::letter::Letter;
use crate::message::Message;
use crate::utils::color::hsl_to_rgb;
use crate::dictionary::Dictionary;
use crate::discovered_word::DiscoveredWord;
use crate::session::Session;

/// Main application state for Words with Toddlers
pub struct WordsWithToddlers {
    letters: Vec<Letter>,
    discovered_words: Vec<DiscoveredWord>,
    dictionary: Dictionary,
    is_fullscreen: bool,
    letters_scroll_id: ScrollableId,
    has_started_typing: bool,
    cursor_visible: bool,
}

impl WordsWithToddlers {
    /// Creates a new instance of the application
    pub fn new() -> (Self, Task<Message>) {
        (
            WordsWithToddlers {
                letters: Vec::new(),
                discovered_words: Vec::new(),
                dictionary: Dictionary::new(),
                is_fullscreen: false,
                letters_scroll_id: ScrollableId::unique(),
                has_started_typing: false,
                cursor_visible: true,
            },
            // Send a message after a short delay to set window to AlwaysOnTop
            Task::perform(
                async {
                    // Small delay to ensure window is created in current Space
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                },
                |_| Message::WindowOpened
            )
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
        }
    }

    /// Builds the user interface
    pub fn view(&self) -> Element<Message> {
        let mut main_column = column![]
            .spacing(20)
            .align_x(alignment::Horizontal::Center);
        
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

    /// Sets up event subscriptions for keyboard input and cursor blinking
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            event::listen_with(|event, _status, _id| match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                    Some(Message::KeyPressed(key))
                }
                _ => None,
            }),
            iced::time::every(std::time::Duration::from_millis(530))
                .map(|_| Message::ToggleCursor),
        ])
    }

    /// Returns the application theme
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    /// Handles keyboard input
    fn handle_key_press(&mut self, key: keyboard::Key) -> Task<Message> {
        match key {
            keyboard::Key::Named(keyboard::key::Named::Escape) => {
                return exit();
            }
            keyboard::Key::Named(keyboard::key::Named::Enter) => {
                // Save session if we have typed anything
                if !self.letters.is_empty() {
                    let typed_text: String = self.letters.iter()
                        .map(|l| l.character)
                        .collect();
                    
                    let session = Session::new(typed_text, self.discovered_words.iter()
                        .map(|w| w.text.clone())
                        .collect());
                    
                    // Try to save the session
                    match session.save() {
                        Ok(path) => eprintln!("Session saved to: {:?}", path),
                        Err(e) => eprintln!("Failed to save session: {}", e),
                    }
                }
                
                // Check for any last word before clearing
                self.check_and_save_word();
                
                // Clear letters and discovered words, mark that we've started typing
                self.letters.clear();
                self.discovered_words.clear();
                self.has_started_typing = true;
            }
            keyboard::Key::Named(keyboard::key::Named::F11) => {
                return Task::done(Message::ToggleFullscreen);
            }
            keyboard::Key::Named(keyboard::key::Named::Backspace) => {
                self.letters.pop();
            }
            keyboard::Key::Character(s) => {
                self.add_character_from_string(s.to_string());
                return scrollable::snap_to(self.letters_scroll_id.clone(), scrollable::RelativeOffset { x: 0.0, y: 1.0 });
            }
            keyboard::Key::Named(keyboard::key::Named::Space) => {
                // Check for word before adding space
                self.check_and_save_word();
                self.add_space();
                return scrollable::snap_to(self.letters_scroll_id.clone(), scrollable::RelativeOffset { x: 0.0, y: 1.0 });
            }
            _ => {}
        }
        Task::none()
    }

    /// Adds a character from the typed string
    fn add_character_from_string(&mut self, s: String) {
        if let Some(c) = s.chars().next() {
            if c.is_alphabetic() || c.is_numeric() {
                let character = if c.is_alphabetic() {
                    c.to_uppercase().next().unwrap()
                } else {
                    c
                };
                self.letters.push(Letter::new(character, self.random_color()));
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
        let last_space_pos = self.letters
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
    fn build_placeholder_screen(&self) -> Element<Message> {
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
    fn build_welcome_screen(&self) -> Element<Message> {
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
        
        let instructions = text("Type as much as you want! Press Space to save words!\nPress Enter to clear all • Escape to exit")
            .size(30)
            .color(Color::from_rgb(0.6, 0.6, 0.6));
        
        container(
            column![welcome_row, instructions]
                .spacing(30)
                .align_x(alignment::Horizontal::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
    }

    /// Builds the display for discovered words with wrapping
    fn build_discovered_words_display(&self) -> Element<Message> {
        // Create a column to hold multiple rows of words
        let mut words_column = column![]
            .spacing(10)
            .align_x(alignment::Horizontal::Center);
        
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
                let mut row_container = Row::new()
                    .spacing(20)
                    .align_y(alignment::Vertical::Center);
                for elem in current_row.drain(..) {
                    row_container = row_container.push(elem);
                }
                words_column = words_column.push(row_container);
                row_width = 0.0;
            }
            
            // Add word to current row
            current_row.push(
                text(&word.text)
                    .size(40)
                    .color(word.color)
                    .into()
            );
            row_width += word_width;
        }
        
        // Add any remaining words in the last row
        if !current_row.is_empty() {
            let mut row_container = Row::new()
                .spacing(20)
                .align_y(alignment::Vertical::Center);
            for elem in current_row {
                row_container = row_container.push(elem);
            }
            words_column = words_column.push(row_container);
        }
        
        container(
            scrollable(words_column)
                .direction(scrollable::Direction::Vertical(
                    scrollable::Scrollbar::default()
                ))
        )
        .width(Length::Fill)
        .height(Length::Shrink)
        .max_height(150) // Limit height to prevent taking too much space
        .padding(10)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(0.1, 0.1, 0.15, 0.5))),
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
    fn build_letters_display(&self) -> Element<Message> {
        // Fixed letter size for consistency
        let letter_size = 120;
        
        // Create a column to hold multiple rows of letters
        let mut letters_column = column![]
            .spacing(10)
            .align_x(alignment::Horizontal::Center);
        
        // Calculate how many letters fit per row based on window width
        let letters_per_row = 15; // Approximate for 1200px width with 120px letters
        
        // Build rows of letters
        let mut current_row = row![]
            .spacing(5)
            .align_y(alignment::Vertical::Center);
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
                current_row = row![]
                    .spacing(5)
                    .align_y(alignment::Vertical::Center);
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
        current_row = current_row.push(
            text("|")
                .size(letter_size)
                .color(cursor_color),
        );

        // Add any remaining letters (and cursor) in the last row
        // Cursor is always present now, so always add the row
        letters_column = letters_column.push(current_row);
        
        // Wrap in scrollable container if content gets too tall
        container(
            scrollable(letters_column)
                .id(self.letters_scroll_id.clone())
                .direction(scrollable::Direction::Vertical(
                    scrollable::Scrollbar::default()
                ))
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
    }

    /// Generates a random bright color for the letters
    fn random_color(&self) -> Color {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let hue = rng.gen_range(0.0..360.0);
        let (r, g, b) = hsl_to_rgb(hue, 0.8, 0.6);
        Color::from_rgb(r, g, b)
    }
}
