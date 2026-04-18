import Foundation

enum GameMode: String, Codable {
    case discovery = "Discovery"
    case challenge = "Challenge"
}

struct Session: Codable {
    let timestamp: Date
    let typedText: String?
    let discoveredWords: [String]?
    let durationSeconds: Int?
    let gameMode: GameMode?
    let gradeLevel: GradeLevel?
    let score: Int?

    init(gameMode: GameMode, gradeLevel: GradeLevel?, score: Int?) {
        self.timestamp = Date()
        self.typedText = nil
        self.discoveredWords = nil
        self.durationSeconds = nil
        self.gameMode = gameMode
        self.gradeLevel = gradeLevel
        self.score = score
    }
}
