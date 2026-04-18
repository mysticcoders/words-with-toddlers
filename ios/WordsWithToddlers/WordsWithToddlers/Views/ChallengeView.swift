import SwiftUI

struct ChallengeView: View {
    @ObservedObject var challenge: WordChallenge
    let onNavigateHome: () -> Void
    @StateObject private var audioPlayer = AudioPlayer()
    @StateObject private var speechService = SpeechService()
    @StateObject private var wordLoader = WordListLoader()
    @State private var inputText: String = ""
    @FocusState private var isTextFieldFocused: Bool

    var body: some View {
        ZStack {
            VStack(spacing: 20) {
                headerView

                Spacer()

                if challenge.mode == .visual || challenge.shouldRevealWord() {
                    wordDisplayView
                } else {
                    replayButtonView
                }

                Spacer()

                typedLettersView

                Spacer()

                invisibleTextField
            }
            .padding()

            if challenge.isCelebrating {
                CelebrationView()
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(Color(UIColor.systemBackground))
        .onAppear {
            if challenge.mode == .audio {
                speechService.speak(challenge.currentWord)
            }
            isTextFieldFocused = true
        }
        .onTapGesture {
            isTextFieldFocused = true
        }
    }

    private var headerView: some View {
        VStack(spacing: 10) {
            HStack {
                Button(action: onNavigateHome) {
                    HStack {
                        Image(systemName: "chevron.left")
                        Text("Back")
                    }
                    .font(.system(size: 18))
                    .padding(.horizontal, 12)
                    .padding(.vertical, 8)
                    .background(Color.gray)
                    .foregroundColor(.white)
                    .cornerRadius(8)
                }

                Spacer()
            }

            HStack {
                Text("Score: \(challenge.score)")
                    .font(.system(size: 24, weight: .medium))

                Spacer()

                Text("Level: \(challenge.gradeLevel.displayName)")
                    .font(.system(size: 24, weight: .medium))
            }
        }
    }

    private var wordDisplayView: some View {
        VStack {
            if challenge.shouldRevealWord() {
                Text("Here's the word to help you:")
                    .font(.system(size: 30))
                    .foregroundColor(Color(red: 1.0, green: 0.7, blue: 0.3))
            }

            Text(challenge.currentWord.uppercased())
                .font(.system(size: 80, weight: .bold))
                .foregroundColor(.blue)
        }
    }

    private var replayButtonView: some View {
        Button(action: {
            speechService.speak(challenge.currentWord)
        }) {
            HStack {
                Text("🔊")
                    .font(.system(size: 40))
                Text("Replay Word")
                    .font(.system(size: 30, weight: .medium))
            }
            .padding()
            .background(Color.blue)
            .foregroundColor(.white)
            .cornerRadius(15)
        }
    }

    private var typedLettersView: some View {
        HStack(spacing: 10) {
            ForEach(challenge.typedLetters) { letter in
                ColoredLetterView(letter: letter, size: 80)
            }
        }
    }

    private var invisibleTextField: some View {
        TextField("", text: $inputText)
            .focused($isTextFieldFocused)
            .opacity(0)
            .frame(height: 0)
            .onChange(of: inputText) { oldValue, newValue in
                if newValue.isEmpty && !oldValue.isEmpty {
                    handleBackspace()
                } else if let lastChar = newValue.last {
                    handleKeyPress(lastChar)
                }
                inputText = ""
            }
    }

    private func handleKeyPress(_ character: Character) {
        guard !challenge.isCelebrating else { return }

        if character.isLetter {
            let letter = Letter(
                character: character.uppercased().first!,
                color: .randomLetterColor()
            )
            challenge.addLetter(letter)

            if challenge.typedText().count == challenge.currentWord.count {
                checkWord()
            }
        }
    }

    private func handleBackspace() {
        guard !challenge.isCelebrating else { return }
        challenge.removeLastLetter()
    }

    private func checkWord() {
        if challenge.checkIfCorrect() {
            challenge.handleCorrectWord()
            audioPlayer.playSound(named: "Swoosh")

            DispatchQueue.main.asyncAfter(deadline: .now() + 1.5) {
                finishCelebration()
            }
        } else {
            challenge.handleIncorrectWord()
            challenge.clearTyped()
        }
    }

    private func finishCelebration() {
        if challenge.shouldLevelUp() {
            if let newLevel = challenge.levelUp(),
               let words = wordLoader.getWords(for: newLevel) {
                challenge.updateWordList(words)
            }
        } else if challenge.shouldLevelDown() {
            if let newLevel = challenge.levelDown(),
               let words = wordLoader.getWords(for: newLevel) {
                challenge.updateWordList(words)
            }
        } else {
            challenge.finishCelebration()
        }

        if challenge.mode == .audio {
            speechService.speak(challenge.currentWord)
        }
    }
}
