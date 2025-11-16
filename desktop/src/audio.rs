use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// Plays a sound effect if no sound is currently playing
pub fn play_sound(sound_playing: Arc<AtomicBool>, sound_path: String) {
    // Check if sound is already playing
    if sound_playing.load(Ordering::SeqCst) {
        return;
    }

    // Set flag to true
    sound_playing.store(true, Ordering::SeqCst);

    // Spawn background thread to play sound
    std::thread::spawn(move || {
        // Load the sound file into memory
        let bytes = match std::fs::read(&sound_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Failed to read sound file {}: {}", sound_path, e);
                sound_playing.store(false, Ordering::SeqCst);
                return;
            }
        };

        // Create decoder from memory (Cursor provides seekable source)
        let cursor = Cursor::new(bytes);
        let source = match Decoder::new(cursor) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to decode sound file: {}", e);
                sound_playing.store(false, Ordering::SeqCst);
                return;
            }
        };

        // Create output stream and sink
        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to create audio output stream: {}", e);
                sound_playing.store(false, Ordering::SeqCst);
                return;
            }
        };

        let sink = match Sink::try_new(&stream_handle) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to create audio sink: {}", e);
                sound_playing.store(false, Ordering::SeqCst);
                return;
            }
        };

        // Play the sound
        sink.append(source);
        sink.sleep_until_end();

        // Reset flag when done
        sound_playing.store(false, Ordering::SeqCst);
    });
}
