use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;

/// Represents a typing session that gets saved to disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub timestamp: String,
    pub typed_text: String,
    pub discovered_words: Vec<String>,
    pub duration_seconds: Option<u64>,
}

impl Session {
    /// Creates a new session with the given text and discovered words
    pub fn new(typed_text: String, discovered_words: Vec<String>) -> Self {
        Session {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            typed_text,
            discovered_words,
            duration_seconds: None,
        }
    }
    
    /// Saves the session to disk in the appropriate directory
    pub fn save(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        // Get the sessions directory
        let sessions_dir = get_sessions_directory()?;
        
        // Create date-based subdirectory
        let date = Local::now();
        let date_dir = sessions_dir.join(date.format("%Y-%m-%d").to_string());
        fs::create_dir_all(&date_dir)?;
        
        // Create filename with timestamp
        let filename = format!("session_{}.json", date.format("%H-%M-%S"));
        let file_path = date_dir.join(filename);
        
        // Serialize session to JSON
        let json = serde_json::to_string_pretty(&self)?;
        
        // Write to file
        let mut file = fs::File::create(&file_path)?;
        file.write_all(json.as_bytes())?;
        
        eprintln!("Session saved to: {:?}", file_path);
        
        Ok(file_path)
    }
}

/// Gets the sessions directory, creating it if it doesn't exist
fn get_sessions_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Use Documents directory for user data
    let documents_dir = dirs::document_dir()
        .ok_or("Could not find Documents directory")?;
    
    let app_dir = documents_dir.join("WordsWithToddlers");
    let sessions_dir = app_dir.join("sessions");
    
    // Create directories if they don't exist
    fs::create_dir_all(&sessions_dir)?;
    
    Ok(sessions_dir)
}

