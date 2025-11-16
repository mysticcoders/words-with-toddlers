use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Manages dictionary operations for word validation
pub struct Dictionary {
    toddler_words: HashSet<String>,
    system_words: HashSet<String>,
}

impl Dictionary {
    /// Creates a new Dictionary by loading both toddler-friendly and system words
    pub fn new() -> Self {
        let mut toddler_words = HashSet::new();
        let mut system_words = HashSet::new();

        // First, load our curated toddler-friendly words
        let toddler_word_list = include_str!("toddler_words.txt");
        for line in toddler_word_list.lines() {
            let line = line.trim();
            // Skip comments and empty lines
            if !line.starts_with('#') && !line.is_empty() {
                toddler_words.insert(line.to_lowercase());
            }
        }

        // Also load system dictionary for additional words (all lengths)
        if let Ok(file) = File::open("/usr/share/dict/words") {
            let reader = BufReader::new(file);
            for word in reader.lines().map_while(Result::ok) {
                let word_lower = word.to_lowercase();
                // Include all words that contain only letters (any length)
                if !word_lower.is_empty() && word_lower.chars().all(|c| c.is_alphabetic()) {
                    system_words.insert(word_lower);
                }
            }
        }

        Dictionary {
            toddler_words,
            system_words,
        }
    }

    /// Checks if a word exists in the dictionary
    pub fn is_valid_word(&self, word: &str) -> bool {
        let word_lower = word.to_lowercase();

        // Allow all word lengths, including single letters like "a" and "I"
        if word_lower.is_empty() {
            return false;
        }

        // Check toddler words first (preferred), then system dictionary
        self.toddler_words.contains(&word_lower) || self.system_words.contains(&word_lower)
    }

    /// Parses compound words from a string, finding all valid words within it
    /// For example: "dogcat" returns ["dog", "cat"]
    pub fn parse_compound_words(&self, text: &str) -> Vec<String> {
        let mut found_words = Vec::new();
        let text_lower = text.to_lowercase();
        let text_chars: Vec<char> = text_lower.chars().filter(|c| c.is_alphabetic()).collect();

        if text_chars.is_empty() {
            return found_words;
        }

        let text_clean: String = text_chars.iter().collect();
        self.parse_recursive(&text_clean, &mut found_words);

        // Return all found words (duplicates are OK)
        found_words
    }

    /// Recursive helper to find all words in a string
    fn parse_recursive(&self, remaining: &str, found: &mut Vec<String>) {
        if remaining.is_empty() {
            return;
        }

        // Try to find the longest valid word starting from the beginning
        let mut found_word = false;

        // Start with longest possible and work down to single letters
        for end in (1..=remaining.len()).rev() {
            let candidate = &remaining[0..end];

            if self.is_valid_word(candidate) {
                found.push(candidate.to_string());
                // Recursively parse the rest
                self.parse_recursive(&remaining[end..], found);
                found_word = true;
                break;
            }
        }

        // If no word found at this position, try starting from next character
        if !found_word && remaining.len() > 1 {
            self.parse_recursive(&remaining[1..], found);
        }
    }
}
