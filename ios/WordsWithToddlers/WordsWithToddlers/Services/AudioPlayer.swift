import AVFoundation

class AudioPlayer: NSObject, ObservableObject {
    private var player: AVAudioPlayer?
    private var isSoundPlaying = false

    func playSound(named soundName: String) {
        guard !isSoundPlaying else { return }

        guard let url = Bundle.main.url(forResource: soundName, withExtension: "wav", subdirectory: "sounds") else {
            print("Sound file not found: \(soundName).wav")
            return
        }

        do {
            player = try AVAudioPlayer(contentsOf: url)
            player?.delegate = self
            isSoundPlaying = true
            player?.play()
        } catch {
            print("Failed to play sound: \(error.localizedDescription)")
            isSoundPlaying = false
        }
    }

    func stopSound() {
        player?.stop()
        isSoundPlaying = false
    }
}

extension AudioPlayer: AVAudioPlayerDelegate {
    func audioPlayerDidFinishPlaying(_ player: AVAudioPlayer, successfully flag: Bool) {
        isSoundPlaying = false
    }
}
