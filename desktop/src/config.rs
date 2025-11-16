use crate::grade_level::GradeLevel;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub selected_sound: String,
    #[serde(default)]
    pub last_selected_grade: GradeLevel,
    #[serde(default = "default_uppercase")]
    pub use_uppercase: bool,
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
        }
    }
}

/// Gets the configuration directory path
fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir()
        .ok_or("Could not find home directory")?;

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
