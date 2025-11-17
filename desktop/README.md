# Words with Toddlers - Desktop App

A toddler-friendly educational application built with Rust and the Iced GUI framework.

## Prerequisites

- Rust toolchain (install from https://rustup.rs/)
- On Linux: `libasound2-dev` package for audio support

## Cargo Commands

### Building the Project

**Development Build**
```bash
cargo build
```
Creates an unoptimized build with debug symbols in `target/debug/`. Faster to compile but slower to run.

**Release Build**
```bash
cargo build --release
```
Creates an optimized build in `target/release/`. Takes longer to compile but runs much faster. Use this for distribution.

**Build macOS .app Bundle**
```bash
cargo install cargo-packager  # First time only
cargo packager --release
```
Creates a complete macOS application bundle at `target/release/bundle/macos/Words with Toddlers.app`. This is a double-clickable .app that can be distributed to other macOS users.

### Running the Application

**Run in Development Mode**
```bash
cargo run
```
Builds and runs the application in debug mode.

**Run in Release Mode**
```bash
cargo run --release
```
Builds and runs the optimized version. Recommended for actual use.

### Code Quality

**Check for Errors Without Building**
```bash
cargo check
```
Quickly checks if the code compiles without producing an executable. Useful for catching errors during development.

**Run Clippy Linter**
```bash
cargo clippy
```
Runs the Rust linter to catch common mistakes and suggest improvements.

**Format Code**
```bash
cargo fmt
```
Automatically formats all Rust code according to standard style guidelines.

### Cleaning Build Artifacts

**Clean All Build Artifacts**
```bash
cargo clean
```
Removes the `target/` directory and all compiled files. Useful when troubleshooting build issues or reclaiming disk space.

### Updating Dependencies

**Update Dependencies**
```bash
cargo update
```
Updates dependencies to their latest compatible versions based on `Cargo.toml` specifications.

**View Dependency Tree**
```bash
cargo tree
```
Shows all dependencies and their versions in a tree structure.

## Platform-Specific Notes

### macOS
- Text-to-speech uses the built-in `say` command
- No additional dependencies required

### Linux
- Requires ALSA development libraries:
  ```bash
  sudo apt-get install libasound2-dev  # Debian/Ubuntu
  sudo dnf install alsa-lib-devel      # Fedora
  ```

### Windows
- Audio should work out of the box
- Text-to-speech feature is macOS-only

## Project Structure

```
desktop/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Main application logic and UI
│   ├── message.rs           # Message types for UI events
│   ├── letter.rs            # Letter display logic
│   ├── word_challenge.rs    # Word challenge game mode
│   ├── tic_tac_toe.rs       # Tic Tac Toe game
│   ├── speech.rs            # Text-to-speech integration
│   ├── audio.rs             # Sound effect playback
│   ├── config.rs            # Configuration persistence
│   └── ...
├── Cargo.toml               # Project dependencies and metadata
└── README.md                # This file
```

## Game Modes

1. **Discovery Mode** - Free typing with colorful letters
2. **See Words** - Visual word challenge with speech
3. **Hear Words** - Audio-only word challenge
4. **Tic Tac Toe** - Two-player game

## Keyboard Shortcuts

- **ESC** - Exit application or return to welcome screen
- **Enter** - Save session and return to welcome (Discovery mode)
- **Space** - Save word (Discovery mode)
- **F11** - Toggle fullscreen
- **1-9** - Make moves in Tic Tac Toe
- **Backspace** - Delete last letter

## Troubleshooting

**Build fails with audio errors on Linux:**
```bash
sudo apt-get install libasound2-dev pkg-config
```

**Clean rebuild:**
```bash
cargo clean
cargo build --release
```

**Check Rust version:**
```bash
rustc --version
```
Recommended: Rust 1.70 or later

## Creating Distributable Builds

### macOS Application Bundle

The project is configured to use `cargo-packager` for creating distributable .app bundles:

```bash
# Install cargo-packager (first time only)
cargo install cargo-packager

# Build the .app bundle
cargo packager --release
```

The resulting application will be at:
```
target/release/bundle/macos/Words with Toddlers.app
```

You can then:
- Double-click to run the app
- Drag to Applications folder
- Distribute to other macOS users (requires code signing for distribution outside local testing)

The bundle configuration is defined in `Cargo.toml` under `[package.metadata.packager]`. The app icon (`app_icon.icns` in the project root) will automatically be included in the bundle.

## Development Tips

- Use `cargo check` frequently while coding for fast feedback
- Run `cargo clippy` before committing to catch common issues
- Use `cargo fmt` to maintain consistent code style
- Build with `--release` when testing performance or user experience
- Use `cargo packager --release` to create a distributable .app bundle
