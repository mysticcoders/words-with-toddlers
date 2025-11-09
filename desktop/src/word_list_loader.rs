use crate::grade_level::GradeLevel;
use std::collections::HashMap;

pub struct WordListLoader {
    words_by_grade: HashMap<GradeLevel, Vec<String>>,
}

impl WordListLoader {
    pub fn new() -> Self {
        let mut words_by_grade = HashMap::new();

        // Load Dolch lists
        words_by_grade.insert(
            GradeLevel::PreK,
            Self::load_words(include_str!("../../shared/word_lists/dolch_pre_primer.txt")),
        );

        words_by_grade.insert(
            GradeLevel::Kindergarten,
            Self::load_words(include_str!("../../shared/word_lists/dolch_primer.txt")),
        );

        // First grade: Dolch First + Fry 1-100
        let mut first_grade_words = Self::load_words(include_str!("../../shared/word_lists/dolch_first.txt"));
        first_grade_words.extend(Self::load_words(include_str!("../../shared/word_lists/fry_001_100.txt")));
        words_by_grade.insert(GradeLevel::First, first_grade_words);

        // Second grade: Dolch Second + Fry 101-200
        let mut second_grade_words = Self::load_words(include_str!("../../shared/word_lists/dolch_second.txt"));
        second_grade_words.extend(Self::load_words(include_str!("../../shared/word_lists/fry_101_200.txt")));
        words_by_grade.insert(GradeLevel::Second, second_grade_words);

        // Third grade: Dolch Third + Fry 201-300
        let mut third_grade_words = Self::load_words(include_str!("../../shared/word_lists/dolch_third.txt"));
        third_grade_words.extend(Self::load_words(include_str!("../../shared/word_lists/fry_201_300.txt")));
        words_by_grade.insert(GradeLevel::Third, third_grade_words);

        // Fourth grade: Fry 301-400
        words_by_grade.insert(
            GradeLevel::Fourth,
            Self::load_words(include_str!("../../shared/word_lists/fry_301_400.txt")),
        );

        // Fifth grade: Fry 401-600 (combining two files)
        let mut fifth_grade_words = Self::load_words(include_str!("../../shared/word_lists/fry_401_500.txt"));
        fifth_grade_words.extend(Self::load_words(include_str!("../../shared/word_lists/fry_501_600.txt")));
        words_by_grade.insert(GradeLevel::Fifth, fifth_grade_words);

        // Sixth grade: Fry 601-1000 (combining four files)
        let mut sixth_grade_words = Self::load_words(include_str!("../../shared/word_lists/fry_601_700.txt"));
        sixth_grade_words.extend(Self::load_words(include_str!("../../shared/word_lists/fry_701_800.txt")));
        sixth_grade_words.extend(Self::load_words(include_str!("../../shared/word_lists/fry_801_900.txt")));
        sixth_grade_words.extend(Self::load_words(include_str!("../../shared/word_lists/fry_901_1000.txt")));
        words_by_grade.insert(GradeLevel::Sixth, sixth_grade_words);

        WordListLoader { words_by_grade }
    }

    fn load_words(content: &str) -> Vec<String> {
        content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.to_lowercase())
            .collect()
    }

    pub fn get_words_for_grade(&self, grade: GradeLevel) -> Option<&Vec<String>> {
        self.words_by_grade.get(&grade)
    }

    pub fn word_count_for_grade(&self, grade: GradeLevel) -> usize {
        self.words_by_grade
            .get(&grade)
            .map(|words| words.len())
            .unwrap_or(0)
    }
}

impl Default for WordListLoader {
    fn default() -> Self {
        Self::new()
    }
}
