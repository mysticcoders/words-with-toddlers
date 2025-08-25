use iced::{
    alignment, event, keyboard, widget::{column, container, row, text}, 
    window, Color, Element, Event, Length, Subscription, Task, Theme, exit
};
use crate::letter::Letter;
use crate::message::Message;
use crate::utils::color::hsl_to_rgb;

/// Main application state for Words with Toddlers
pub struct WordsWithToddlers {
    letters: Vec<Letter>,
    is_fullscreen: bool,
}

impl WordsWithToddlers {
    /// Creates a new instance of the application
    pub fn new() -> (Self, Task<Message>) {
        (
            WordsWithToddlers {
                letters: Vec::new(),
                is_fullscreen: false,
            },
            Task::none()
        )
    }

    /// Handles all application messages and updates state accordingly
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::KeyPressed(key) => self.handle_key_press(key),
            Message::ToggleFullscreen => self.toggle_fullscreen(),
        }
    }

    /// Builds the user interface
    pub fn view(&self) -> Element<Message> {
        let content = if self.letters.is_empty() {
            self.build_welcome_screen()
        } else {
            self.build_letters_display()
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.05, 0.05, 0.1))),
                ..Default::default()
            })
            .into()
    }

    /// Sets up event subscriptions for keyboard input
    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _id| {
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                    Some(Message::KeyPressed(key))
                }
                _ => None
            }
        })
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
                self.letters.clear();
            }
            keyboard::Key::Named(keyboard::key::Named::F11) => {
                return Task::done(Message::ToggleFullscreen);
            }
            keyboard::Key::Named(keyboard::key::Named::Backspace) => {
                self.letters.pop();
            }
            keyboard::Key::Character(s) => {
                self.add_character_from_string(s.to_string());
            }
            keyboard::Key::Named(keyboard::key::Named::Space) => {
                self.add_space();
            }
            _ => {}
        }
        Task::none()
    }

    /// Adds a character from the typed string
    fn add_character_from_string(&mut self, s: String) {
        if self.letters.len() < 25 {
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
    }

    /// Adds a space character
    fn add_space(&mut self) {
        if self.letters.len() < 25 {
            self.letters.push(Letter::new(' ', self.random_color()));
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
            }
        )
    }

    /// Builds the welcome screen with multicolored title
    fn build_welcome_screen(&self) -> Element<Message> {
        let welcome_text = "Welcome to Words With Toddlers!";
        let mut welcome_row = row![]
            .spacing(5)
            .align_y(alignment::Vertical::Center);
        
        for (i, ch) in welcome_text.chars().enumerate() {
            let hue = (i as f32 * 360.0 / welcome_text.len() as f32) % 360.0;
            let (r, g, b) = hsl_to_rgb(hue, 0.8, 0.6);
            welcome_row = welcome_row.push(
                text(ch.to_string())
                    .size(50)
                    .color(Color::from_rgb(r, g, b))
            );
        }
        
        let instructions = text("Type up to 25 letters or numbers!\nPress Enter to clear • Escape to exit")
            .size(30)
            .color(Color::from_rgb(0.6, 0.6, 0.6));
        
        container(
            column![welcome_row, instructions]
                .spacing(30)
                .align_x(alignment::Horizontal::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
    }

    /// Builds the display for typed letters
    fn build_letters_display(&self) -> Element<Message> {
        let letter_size = self.calculate_letter_size();
        
        let mut letters_row = row![]
            .spacing(10)
            .align_y(alignment::Vertical::Center);

        for letter in &self.letters {
            letters_row = letters_row.push(
                text(letter.character.to_string())
                    .size(letter_size)
                    .color(letter.color)
            );
        }

        container(letters_row)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    /// Calculates appropriate text size based on number of letters
    fn calculate_letter_size(&self) -> u16 {
        match self.letters.len() {
            1..=3 => 300,
            4..=6 => 200,
            7..=10 => 150,
            11..=15 => 100,
            16..=20 => 80,
            _ => 60,
        }
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