import SwiftUI

struct DiscoveryView: View {
    let onNavigateHome: () -> Void
    @State private var letters: [Letter] = []
    @State private var discoveredWords: [DiscoveredWord] = []
    @State private var inputText: String = ""
    @FocusState private var isTextFieldFocused: Bool
    @StateObject private var audioPlayer = AudioPlayer()
    @StateObject private var dictionaryService = DictionaryService()
    @StateObject private var configService = ConfigService()
    @State private var config: AppConfig = .default

    var body: some View {
        VStack {
            HStack {
                Button(action: onNavigateHome) {
                    HStack {
                        Image(systemName: "chevron.left")
                        Text("Back")
                    }
                    .font(.system(size: 20))
                    .padding()
                    .background(Color.gray)
                    .foregroundColor(.white)
                    .cornerRadius(10)
                }

                Spacer()

                Button(action: clearAll) {
                    Text("Clear")
                        .font(.system(size: 20))
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }
            }
            .padding()

            ScrollView {
                LazyVStack(alignment: .leading, spacing: 10) {
                    ForEach(letters) { letter in
                        ColoredLetterView(letter: letter, size: 120)
                    }
                }
                .padding()
            }

            if !discoveredWords.isEmpty {
                DiscoveredWordsView(words: discoveredWords)
                    .frame(height: 150)
            }

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
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(Color(UIColor.systemBackground))
        .onAppear {
            config = configService.loadConfig()
            isTextFieldFocused = true
        }
        .onTapGesture {
            isTextFieldFocused = true
        }
    }

    private func handleKeyPress(_ character: Character) {
        if character.isLetter {
            let letter = Letter(
                character: character.uppercased().first!,
                color: .randomLetterColor()
            )
            letters.append(letter)
        } else if character == " " {
            let currentText = letters.map { String($0.character) }.joined()
            if dictionaryService.isValidWord(currentText) {
                discoveredWords.append(DiscoveredWord(text: currentText))
            }
            letters.append(Letter(character: " ", color: .clear))
        }
    }

    private func handleBackspace() {
        if !letters.isEmpty {
            letters.removeLast()
        }
    }

    private func clearAll() {
        audioPlayer.playSound(named: config.selectedSound)
        letters.removeAll()
        discoveredWords.removeAll()
    }
}
