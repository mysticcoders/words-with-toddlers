/// Represents a system sound option
#[derive(Debug, Clone)]
pub struct SystemSound {
    pub name: &'static str,
    pub display_name: &'static str,
    pub path: &'static str,
}

/// Curated list of favorite system sounds
pub const SOUNDS: &[SystemSound] = &[
    SystemSound {
        name: "Swoosh",
        display_name: "Swoosh",
        path: "sounds/Swoosh.wav",
    },
    SystemSound {
        name: "Swish",
        display_name: "Swish",
        path: "sounds/Swish.wav",
    },
    SystemSound {
        name: "Tri-Tone",
        display_name: "Tri-Tone",
        path: "sounds/Tri-Tone.wav",
    },
    SystemSound {
        name: "Pop",
        display_name: "Pop",
        path: "/System/Library/Sounds/Pop.aiff",
    },
    SystemSound {
        name: "Tink",
        display_name: "Tink",
        path: "/System/Library/Sounds/Tink.aiff",
    },
    SystemSound {
        name: "Glass",
        display_name: "Glass",
        path: "/System/Library/Sounds/Glass.aiff",
    },
    SystemSound {
        name: "Chime",
        display_name: "Chime",
        path: "sounds/Chime.wav",
    },
    SystemSound {
        name: "Bell",
        display_name: "Bell",
        path: "sounds/Bell.wav",
    },
    SystemSound {
        name: "Ding",
        display_name: "Ding",
        path: "sounds/Ding.wav",
    },
    SystemSound {
        name: "Hero",
        display_name: "Hero",
        path: "/System/Library/Sounds/Hero.aiff",
    },
];

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
