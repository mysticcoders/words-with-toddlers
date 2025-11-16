use iced::Color;

/// Represents a word that was successfully typed by the toddler
#[derive(Debug, Clone)]
pub struct DiscoveredWord {
    pub text: String,
    pub color: Color,
}

impl DiscoveredWord {
    /// Creates a new discovered word with the given text and color
    pub fn new(text: String, color: Color) -> Self {
        DiscoveredWord { text, color }
    }
}
