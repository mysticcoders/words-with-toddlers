use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum GradeLevel {
    #[default]
    PreK,
    Kindergarten,
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
}

impl GradeLevel {
    #[allow(dead_code)]
    pub fn all() -> Vec<GradeLevel> {
        vec![
            GradeLevel::PreK,
            GradeLevel::Kindergarten,
            GradeLevel::First,
            GradeLevel::Second,
            GradeLevel::Third,
            GradeLevel::Fourth,
            GradeLevel::Fifth,
            GradeLevel::Sixth,
        ]
    }

    pub fn display_name(&self) -> &str {
        match self {
            GradeLevel::PreK => "Pre-K",
            GradeLevel::Kindergarten => "Kindergarten",
            GradeLevel::First => "1st Grade",
            GradeLevel::Second => "2nd Grade",
            GradeLevel::Third => "3rd Grade",
            GradeLevel::Fourth => "4th Grade",
            GradeLevel::Fifth => "5th Grade",
            GradeLevel::Sixth => "6th Grade",
        }
    }

    #[allow(dead_code)]
    pub fn short_name(&self) -> &str {
        match self {
            GradeLevel::PreK => "Pre-K",
            GradeLevel::Kindergarten => "K",
            GradeLevel::First => "1st",
            GradeLevel::Second => "2nd",
            GradeLevel::Third => "3rd",
            GradeLevel::Fourth => "4th",
            GradeLevel::Fifth => "5th",
            GradeLevel::Sixth => "6th",
        }
    }

    pub fn next_level(&self) -> Option<GradeLevel> {
        match self {
            GradeLevel::PreK => Some(GradeLevel::Kindergarten),
            GradeLevel::Kindergarten => Some(GradeLevel::First),
            GradeLevel::First => Some(GradeLevel::Second),
            GradeLevel::Second => Some(GradeLevel::Third),
            GradeLevel::Third => Some(GradeLevel::Fourth),
            GradeLevel::Fourth => Some(GradeLevel::Fifth),
            GradeLevel::Fifth => Some(GradeLevel::Sixth),
            GradeLevel::Sixth => None,
        }
    }

    pub fn previous_level(&self) -> Option<GradeLevel> {
        match self {
            GradeLevel::PreK => None,
            GradeLevel::Kindergarten => Some(GradeLevel::PreK),
            GradeLevel::First => Some(GradeLevel::Kindergarten),
            GradeLevel::Second => Some(GradeLevel::First),
            GradeLevel::Third => Some(GradeLevel::Second),
            GradeLevel::Fourth => Some(GradeLevel::Third),
            GradeLevel::Fifth => Some(GradeLevel::Fourth),
            GradeLevel::Sixth => Some(GradeLevel::Fifth),
        }
    }
}

