import SwiftUI

struct SettingsView: View {
    @StateObject private var configService = ConfigService()
    @StateObject private var audioPlayer = AudioPlayer()
    @State private var config: AppConfig = .default
    @Environment(\.dismiss) private var dismiss

    let availableSounds = ["Swoosh", "Swish", "Tri-Tone", "Chime", "Bell", "Ding"]

    var body: some View {
        NavigationView {
            Form {
                Section(header: Text("Sound Effects")) {
                    Picker("Sound", selection: $config.selectedSound) {
                        ForEach(availableSounds, id: \.self) { sound in
                            Text(sound).tag(sound)
                        }
                    }
                    .onChange(of: config.selectedSound) { _, newValue in
                        audioPlayer.playSound(named: newValue)
                        configService.saveConfig(config)
                    }

                    Button("Test Sound") {
                        audioPlayer.playSound(named: config.selectedSound)
                    }
                }
            }
            .navigationTitle("Settings")
            .navigationBarItems(trailing: Button("Done") {
                dismiss()
            })
        }
        .onAppear {
            config = configService.loadConfig()
        }
    }
}
