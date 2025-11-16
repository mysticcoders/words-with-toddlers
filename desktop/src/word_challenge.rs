use crate::grade_level::GradeLevel;
use crate::letter::Letter;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChallengeMode {
    Visual,
    Audio,
}

#[derive(Debug, Clone)]
pub struct WordChallenge {
    pub grade_level: GradeLevel,
    pub current_word: String,
    pub typed_letters: Vec<Letter>,
    pub score: usize,
    pub words_completed: usize,
    available_words: Vec<String>,
    pub is_celebrating: bool,
    pub mode: ChallengeMode,
    recent_attempts: VecDeque<bool>,
    attempts_since_level_change: usize,
    current_word_wrong_attempts: usize,
    completed_words: HashSet<String>,
}

impl WordChallenge {
    pub fn new(mode: ChallengeMode, words: Vec<String>) -> Self {
        let mut challenge = WordChallenge {
            grade_level: GradeLevel::PreK,
            current_word: String::new(),
            typed_letters: Vec::new(),
            score: 0,
            words_completed: 0,
            available_words: words,
            is_celebrating: false,
            mode,
            recent_attempts: VecDeque::with_capacity(10),
            attempts_since_level_change: 0,
            current_word_wrong_attempts: 0,
            completed_words: HashSet::new(),
        };
        challenge.next_word();
        challenge
    }

    pub fn next_word(&mut self) {
        if !self.available_words.is_empty() {
            let mut rng = thread_rng();

            // Filter out completed words
            let remaining_words: Vec<&String> = self
                .available_words
                .iter()
                .filter(|w| !self.completed_words.contains(*w))
                .collect();

            // If all words completed, clear the completed set to restart
            if remaining_words.is_empty() {
                self.completed_words.clear();
                if let Some(word) = self.available_words.choose(&mut rng) {
                    self.current_word = word.clone();
                }
            } else {
                if let Some(word) = remaining_words.choose(&mut rng) {
                    self.current_word = (*word).clone();
                }
            }
        }
        self.typed_letters.clear();
        self.current_word_wrong_attempts = 0;
    }

    pub fn typed_text(&self) -> String {
        self.typed_letters
            .iter()
            .map(|letter| letter.character.to_lowercase().to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn check_if_correct(&self) -> bool {
        self.typed_text() == self.current_word.to_lowercase()
    }

    pub fn handle_correct_word(&mut self) {
        self.score += 1;
        self.words_completed += 1;
        self.is_celebrating = true;
        self.record_attempt(true);

        // Add to completed words so it won't repeat in this session
        self.completed_words.insert(self.current_word.clone());
    }

    pub fn handle_incorrect_word(&mut self) {
        self.record_attempt(false);
        self.current_word_wrong_attempts += 1;
    }

    pub fn should_reveal_word(&self) -> bool {
        self.mode == ChallengeMode::Audio && self.current_word_wrong_attempts >= 3
    }

    fn record_attempt(&mut self, correct: bool) {
        self.recent_attempts.push_back(correct);
        if self.recent_attempts.len() > 10 {
            self.recent_attempts.pop_front();
        }
        self.attempts_since_level_change += 1;
    }

    pub fn calculate_accuracy(&self) -> f32 {
        if self.recent_attempts.is_empty() {
            return 1.0;
        }
        let correct_count = self.recent_attempts.iter().filter(|&&c| c).count();
        correct_count as f32 / self.recent_attempts.len() as f32
    }

    pub fn should_level_up(&self) -> bool {
        if self.recent_attempts.len() >= 10 && self.attempts_since_level_change >= 10 {
            self.calculate_accuracy() >= 0.8
        } else {
            false
        }
    }

    pub fn should_level_down(&self) -> bool {
        if self.recent_attempts.len() >= 10 && self.attempts_since_level_change >= 10 {
            self.calculate_accuracy() < 0.5
        } else {
            false
        }
    }

    pub fn level_up(&mut self) -> Option<GradeLevel> {
        self.grade_level = self.grade_level.next_level()?;
        self.attempts_since_level_change = 0;
        Some(self.grade_level)
    }

    pub fn level_down(&mut self) -> Option<GradeLevel> {
        self.grade_level = self.grade_level.previous_level()?;
        self.attempts_since_level_change = 0;
        Some(self.grade_level)
    }

    pub fn update_word_list(&mut self, words: Vec<String>) {
        self.available_words = words;
        self.next_word();
    }

    pub fn finish_celebration(&mut self) {
        self.is_celebrating = false;
        self.next_word();
    }

    pub fn add_letter(&mut self, letter: Letter) {
        self.typed_letters.push(letter);
    }

    pub fn remove_last_letter(&mut self) {
        self.typed_letters.pop();
    }

    pub fn clear_typed(&mut self) {
        self.typed_letters.clear();
    }
}
