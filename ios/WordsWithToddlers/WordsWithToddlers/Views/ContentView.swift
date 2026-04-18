import SwiftUI

enum Screen {
    case welcome
    case discovery
    case challenge
    case settings
}

struct ContentView: View {
    @State private var currentScreen: Screen = .welcome
    @State private var wordChallenge: WordChallenge?
    @StateObject private var wordLoader = WordListLoader()

    var body: some View {
        Group {
            switch currentScreen {
            case .welcome:
                WelcomeView(
                    onStartVisualChallenge: startVisualChallenge,
                    onStartAudioChallenge: startAudioChallenge,
                    onStartDiscovery: startDiscovery,
                    onOpenSettings: openSettings
                )
            case .discovery:
                DiscoveryView(onNavigateHome: navigateToWelcome)
            case .challenge:
                if let challenge = wordChallenge {
                    ChallengeView(challenge: challenge, onNavigateHome: navigateToWelcome)
                }
            case .settings:
                SettingsView()
            }
        }
    }

    private func startVisualChallenge() {
        if let words = wordLoader.getWords(for: .preK) {
            wordChallenge = WordChallenge(mode: .visual, words: words)
            currentScreen = .challenge
        }
    }

    private func startAudioChallenge() {
        if let words = wordLoader.getWords(for: .preK) {
            wordChallenge = WordChallenge(mode: .audio, words: words)
            currentScreen = .challenge
        }
    }

    private func startDiscovery() {
        currentScreen = .discovery
    }

    private func openSettings() {
        currentScreen = .settings
    }

    private func navigateToWelcome() {
        currentScreen = .welcome
    }
}
