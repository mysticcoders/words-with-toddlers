use std::process::Command;

pub fn speak_word(word: &str) {
    if cfg!(target_os = "macos") {
        let _ = Command::new("say").arg(word).spawn();
    } else {
        eprintln!("Text-to-speech is only supported on macOS");
    }
}

pub fn speak_word_async(word: String) {
    std::thread::spawn(move || {
        speak_word(&word);
    });
}
