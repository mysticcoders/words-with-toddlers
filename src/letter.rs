use iced::Color;

/// Represents a single letter with its associated color
#[derive(Debug, Clone)]
pub struct Letter {
    pub character: char,
    pub color: Color,
}

impl Letter {
    /// Creates a new Letter with the given character and color
    pub fn new(character: char, color: Color) -> Self {
        Letter { character, color }
    }
}