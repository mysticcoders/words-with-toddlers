# 🌈 <span style="color:#FF6B6B">W</span><span style="color:#4ECDC4">o</span><span style="color:#45B7D1">r</span><span style="color:#96CEB4">d</span><span style="color:#FFEAA7">s</span> <span style="color:#DDA0DD">W</span><span style="color:#98D8C8">i</span><span style="color:#F7DC6F">t</span><span style="color:#85C1E2">h</span> <span style="color:#F8B739">T</span><span style="color:#52B788">o</span><span style="color:#E76F51">d</span><span style="color:#A8DADC">d</span><span style="color:#F1FA8C">l</span><span style="color:#FFB6C1">e</span><span style="color:#87CEEB">r</span><span style="color:#DDA0DD">s</span><span style="color:#98FB98">!</span> 🎨

A fun, colorful, and toddler-safe typing application with adaptive learning capabilities. Available on Desktop (macOS, Windows, Linux) and iOS (iPad, iPhone)!

![Words with Toddlers Screenshot](screenshot.png)

## ✨ Features

### Core Features
- 🔤 **Large, Colorful Letters**: Each typed character appears in huge, randomly colored text
- 🎯 **Two Game Modes**:
  - **Discovery Mode**: Free typing with colorful letters
  - **Challenge Mode**: Grade-level word challenges with adaptive difficulty
- 👁️ **Visual Challenge**: See the word and type it
- 🔊 **Audio Challenge**: Hear the word spoken, then type it (with 3-strike reveal)
- 📚 **Educational Word Lists**:
  - Dolch word lists (220 words + 95 nouns)
  - Fry word lists (1000 high-frequency words)
  - Organized by grade level (Pre-K through 6th grade)
- 🎓 **Adaptive Difficulty**: Automatically levels up/down based on performance
  - Tracks last 10 attempts
  - 80% accuracy → level up
  - <50% accuracy → level down
- 🎉 **Celebration Animations**: Reward correct answers
- 🔒 **Toddler-Safe**: Escape key to exit, always-on-top window

### Platform-Specific Features
- **Desktop**: Text-to-speech via macOS `say` command, customizable sounds
- **iOS**: Native AVSpeechSynthesizer, iPad split-screen support, iPhone portrait mode

## 🏗️ Repository Structure

```
words-with-toddlers/
├── desktop/              # Rust/Iced desktop implementation
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── app.rs
│       ├── models/
│       ├── views/
│       └── services/
├── ios/                  # Swift/SwiftUI iOS implementation
│   └── WordsWithToddlers/
│       ├── Models/
│       ├── Views/
│       └── Services/
├── shared/               # Shared resources
│   ├── word_lists/       # Dolch & Fry word lists (.txt)
│   └── sounds/           # Audio files (.wav)
├── api/                  # Future API server
│   └── spec/
│       └── openapi.yaml  # API specification
└── README.md
```

## 🚀 Installation

### Desktop (Rust/Iced)

#### Prerequisites
- Rust 1.70 or later
- Cargo

#### Build and Run
```bash
# Navigate to desktop directory
cd desktop

# Build in release mode
cargo build --release

# Run the application
cargo run --release
```

#### Desktop Usage
1. Launch the app: `cargo run --release` from `desktop/` directory
2. Choose a mode:
   - Click "👁️ See Words" for visual challenge
   - Click "🔊 Hear Words" for audio challenge
   - Type any letter to start Discovery mode
3. In Challenge mode:
   - Type the displayed/spoken word
   - Score increases with correct answers
   - Automatic difficulty adjustment
4. Press Escape to exit

### iOS (Swift/SwiftUI)

#### Prerequisites
- macOS with Xcode 15 or later
- iOS 17+ for deployment

#### Setup
```bash
# The word lists and sounds need to be added to the Xcode project
# 1. Open Xcode
# 2. Create new iOS App project in ios/WordsWithToddlers/
# 3. Add all Swift files from ios/WordsWithToddlers/WordsWithToddlers/
# 4. Add word lists from shared/word_lists/ to Resources
# 5. Add sounds from shared/sounds/ to Resources
```

#### Manual Xcode Project Creation
1. Open Xcode
2. File → New → Project
3. Select "iOS" → "App"
4. Product Name: "WordsWithToddlers"
5. Interface: SwiftUI
6. Language: Swift
7. Save in `ios/` directory
8. Add all `.swift` files from the following directories:
   - `ios/WordsWithToddlers/WordsWithToddlers/Models/`
   - `ios/WordsWithToddlers/WordsWithToddlers/Views/`
   - `ios/WordsWithToddlers/WordsWithToddlers/Services/`
9. Add `shared/word_lists/*.txt` to project (Copy items if needed)
10. Add `shared/sounds/*.wav` to project (Copy items if needed)
11. Build and run on simulator or device

#### iOS Usage
1. Launch app on iPhone or iPad
2. Choose mode:
   - "👁️ See Words" - Visual word challenge
   - "🔊 Hear Words" - Audio word challenge
3. Type using on-screen keyboard
4. Score and level displayed at top
5. Automatic progression through grade levels

## 🎓 Adaptive Learning System

The challenge mode features an intelligent difficulty system:

- **Initial Level**: Pre-K (40 simple words)
- **Performance Tracking**: Last 10 word attempts
- **Level Up**: ≥80% accuracy over 10 attempts → advance to next grade
- **Level Down**: <50% accuracy over 10 attempts → return to previous grade
- **Grade Progression**: Pre-K → K → 1st → 2nd → 3rd → 4th → 5th → 6th

### Audio Mode Safety Net
When in audio mode, if a child gets the same word wrong 3 times, the word is revealed to help them progress.

## 📚 Word Lists

### Dolch Lists
- **Pre-Primer**: 40 words (Pre-K)
- **Primer**: 52 words (Kindergarten)
- **First**: 41 words
- **Second**: 46 words
- **Third**: 41 words
- **Nouns**: 95 words

### Fry Lists
- **1-100**: Combined with Dolch First (1st grade)
- **101-200**: Combined with Dolch Second (2nd grade)
- **201-300**: Combined with Dolch Third (3rd grade)
- **301-400**: 4th grade
- **401-600**: 5th grade
- **601-1000**: 6th grade

## 🔐 Safety Features

### Desktop Toddler Safety
- Escape key to exit (harder for toddlers to find)
- Window stays always-on-top
- No external links or dangerous actions

### macOS Guided Access (Recommended)
1. Go to **System Settings → Accessibility → Guided Access**
2. Enable Guided Access and set a passcode
3. Launch Words with Toddlers
4. Triple-click Touch ID/power button to activate
5. Mac is locked to just this app until you unlock

### iOS Guided Access
1. Go to **Settings → Accessibility → Guided Access**
2. Enable Guided Access and set a passcode
3. Launch Words with Toddlers
4. Triple-click Home/Side button to activate
5. Device is locked to just this app until you unlock

## 🛠️ Technical Details

### Desktop Stack
- **Language**: Rust
- **GUI Framework**: Iced 0.13
- **Audio**: rodio
- **Text-to-Speech**: macOS `say` command
- **Architecture**: Elm-style message passing

### iOS Stack
- **Language**: Swift
- **UI Framework**: SwiftUI
- **Audio**: AVAudioPlayer
- **Text-to-Speech**: AVSpeechSynthesizer
- **Architecture**: MVVM with ObservableObject

### Shared Data Models
Both platforms implement identical models:
- `GradeLevel`: 8 grade levels with progression logic
- `WordChallenge`: Adaptive difficulty tracking
- `Letter`: Character with random color
- `ChallengeMode`: Visual or Audio

## 🌐 Future API Server

An API specification is defined in `api/spec/openapi.yaml` for future cloud features:
- User accounts and progress tracking
- Cross-device session sync
- Leaderboards
- Dynamic word list updates

Endpoints (planned):
- `GET /words/{gradeLevel}` - Fetch word lists
- `POST /sessions` - Save session data
- `GET /users/{userId}/progress` - Sync progress

## 📝 License

This project is open source and available under the MIT License.

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest new features
- Submit pull requests for desktop or iOS

## 👶 Perfect For

- Teaching letter recognition
- Grade-level sight word practice
- Adaptive learning based on child's skill
- Audio/visual learning styles
- Building typing confidence
- Color and sound recognition

## 🙏 Acknowledgments

- Iced GUI framework developers
- SwiftUI community
- Dolch and Fry word list creators
- Rust and Swift communities
- Parents everywhere teaching their toddlers!

---

Made with ❤️ by [Mystic](https://wecodefire.com)
