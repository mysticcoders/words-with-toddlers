/// Represents a system sound option
#[derive(Debug, Clone)]
pub struct SystemSound {
    pub name: &'static str,
    pub display_name: &'static str,
    pub path: &'static str,
}

/// Curated list of available wav sounds
pub const SOUNDS: &[SystemSound] = &[
    SystemSound {
        name: "Swoosh",
        display_name: "Swoosh",
        path: "../shared/sounds/Swoosh.wav",
    },
    SystemSound {
        name: "Swish",
        display_name: "Swish",
        path: "../shared/sounds/Swish.wav",
    },
    SystemSound {
        name: "Tri-Tone",
        display_name: "Tri-Tone",
        path: "../shared/sounds/Tri-Tone.wav",
    },
    SystemSound {
        name: "Chime",
        display_name: "Chime",
        path: "../shared/sounds/Chime.wav",
    },
    SystemSound {
        name: "Bell",
        display_name: "Bell",
        path: "../shared/sounds/Bell.wav",
    },
    SystemSound {
        name: "Ding",
        display_name: "Ding",
        path: "../shared/sounds/Ding.wav",
    },
];

/// Typewriter sound for keystroke mode
#[allow(dead_code)]
pub const TYPEWRITER_SOUND: &str = "../shared/sounds/typewriter-key.wav";

/// Get the default sound (Swoosh)
pub fn default_sound() -> &'static SystemSound {
    &SOUNDS[0] // Swoosh is first
}

/// Get a sound by name
pub fn get_sound_by_name(name: &str) -> Option<&'static SystemSound> {
    SOUNDS.iter().find(|s| s.name == name)
}

/// Get the path for a sound by name, or default to Swoosh
pub fn get_sound_path(name: &str) -> &'static str {
    get_sound_by_name(name)
        .map(|s| s.path)
        .unwrap_or_else(|| default_sound().path)
}
