use crate::grade_level::GradeLevel;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Represents a color palette for letter colors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColorPalette {
    Rainbow,
    PinkShades,
    BlueShades,
    GreenShades,
    WarmTones,
    CoolTones,
    Pastels,
    Neon,
}

impl Default for ColorPalette {
    fn default() -> Self {
        ColorPalette::Rainbow
    }
}

impl ColorPalette {
    /// Returns the display name for the palette
    pub fn display_name(&self) -> &str {
        match self {
            ColorPalette::Rainbow => "Rainbow",
            ColorPalette::PinkShades => "Pink Shades",
            ColorPalette::BlueShades => "Blue Shades",
            ColorPalette::GreenShades => "Green Shades",
            ColorPalette::WarmTones => "Warm Tones",
            ColorPalette::CoolTones => "Cool Tones",
            ColorPalette::Pastels => "Pastels",
            ColorPalette::Neon => "Neon",
        }
    }

    /// Returns the hue range, saturation, and lightness for generating colors
    pub fn color_params(&self) -> (f32, f32, f32, f32) {
        match self {
            ColorPalette::Rainbow => (0.0, 360.0, 0.8, 0.6),
            ColorPalette::PinkShades => (300.0, 360.0, 0.75, 0.65),
            ColorPalette::BlueShades => (190.0, 260.0, 0.8, 0.6),
            ColorPalette::GreenShades => (90.0, 170.0, 0.7, 0.55),
            ColorPalette::WarmTones => (0.0, 60.0, 0.85, 0.6),
            ColorPalette::CoolTones => (200.0, 310.0, 0.75, 0.6),
            ColorPalette::Pastels => (0.0, 360.0, 0.5, 0.75),
            ColorPalette::Neon => (0.0, 360.0, 1.0, 0.5),
        }
    }

    /// Returns a representative preview color (hue) for the palette
    pub fn preview_color(&self) -> (f32, f32, f32) {
        let (hue_min, hue_max, sat, lit) = self.color_params();
        let mid_hue = (hue_min + hue_max) / 2.0;
        crate::utils::color::hsl_to_rgb(mid_hue, sat, lit)
    }

    /// Returns all available palettes
    pub fn all() -> &'static [ColorPalette] {
        &[
            ColorPalette::Rainbow,
            ColorPalette::PinkShades,
            ColorPalette::BlueShades,
            ColorPalette::GreenShades,
            ColorPalette::WarmTones,
            ColorPalette::CoolTones,
            ColorPalette::Pastels,
            ColorPalette::Neon,
        ]
    }
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub selected_sound: String,
    #[serde(default)]
    pub last_selected_grade: GradeLevel,
    #[serde(default = "default_uppercase")]
    pub use_uppercase: bool,
    #[serde(default)]
    pub kiosk_mode_enabled: bool,
    #[serde(default)]
    pub color_palette: ColorPalette,
}

fn default_uppercase() -> bool {
    true
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            selected_sound: "Swoosh".to_string(),
            last_selected_grade: GradeLevel::default(),
            use_uppercase: true,
            kiosk_mode_enabled: false,
            color_palette: ColorPalette::default(),
        }
    }
}

/// Gets the configuration directory path
fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;

    let config_dir = home_dir
        .join("Library")
        .join("Application Support")
        .join("WordsWithToddlers");

    // Create directory if it doesn't exist
    fs::create_dir_all(&config_dir)?;

    Ok(config_dir)
}

/// Gets the full path to the config file
fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(get_config_dir()?.join("config.json"))
}

/// Loads the application configuration
pub fn load_config() -> AppConfig {
    match try_load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config (using defaults): {}", e);
            AppConfig::default()
        }
    }
}

/// Attempts to load the configuration, returning an error if it fails
fn try_load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(AppConfig::default());
    }

    let contents = fs::read_to_string(&config_path)?;
    let config: AppConfig = serde_json::from_str(&contents)?;

    Ok(config)
}

/// Saves the application configuration
pub fn save_config(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    let contents = serde_json::to_string_pretty(config)?;
    fs::write(&config_path, contents)?;
    Ok(())
}
