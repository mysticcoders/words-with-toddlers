import Foundation

class WordChallenge: ObservableObject {
    @Published var gradeLevel: GradeLevel
    @Published var currentWord: String
    @Published var typedLetters: [Letter]
    @Published var score: Int
    @Published var wordsCompleted: Int
    @Published var isCelebrating: Bool
    @Published var mode: ChallengeMode

    private var availableWords: [String]
    private var recentAttempts: [Bool] = []
    private var attemptsSinceLevelChange: Int = 0
    private var currentWordWrongAttempts: Int = 0

    private let maxRecentAttempts = 10
    private let levelUpAccuracy: Double = 0.8
    private let levelDownAccuracy: Double = 0.5

    init(mode: ChallengeMode, words: [String]) {
        self.mode = mode
        self.gradeLevel = .preK
        self.currentWord = ""
        self.typedLetters = []
        self.score = 0
        self.wordsCompleted = 0
        self.availableWords = words
        self.isCelebrating = false

        nextWord()
    }

    func nextWord() {
        if let word = availableWords.randomElement() {
            currentWord = word
        }
        typedLetters.removeAll()
        currentWordWrongAttempts = 0
    }

    func typedText() -> String {
        typedLetters.map { String($0.character).lowercased() }.joined()
    }

    func checkIfCorrect() -> Bool {
        typedText() == currentWord.lowercased()
    }

    func handleCorrectWord() {
        score += 1
        wordsCompleted += 1
        isCelebrating = true
        recordAttempt(correct: true)
    }

    func handleIncorrectWord() {
        recordAttempt(correct: false)
        currentWordWrongAttempts += 1
    }

    func shouldRevealWord() -> Bool {
        mode == .audio && currentWordWrongAttempts >= 3
    }

    private func recordAttempt(correct: Bool) {
        recentAttempts.append(correct)
        if recentAttempts.count > maxRecentAttempts {
            recentAttempts.removeFirst()
        }
        attemptsSinceLevelChange += 1
    }

    func calculateAccuracy() -> Double {
        guard !recentAttempts.isEmpty else { return 1.0 }
        let correctCount = recentAttempts.filter { $0 }.count
        return Double(correctCount) / Double(recentAttempts.count)
    }

    func shouldLevelUp() -> Bool {
        recentAttempts.count >= maxRecentAttempts &&
        attemptsSinceLevelChange >= maxRecentAttempts &&
        calculateAccuracy() >= levelUpAccuracy
    }

    func shouldLevelDown() -> Bool {
        recentAttempts.count >= maxRecentAttempts &&
        attemptsSinceLevelChange >= maxRecentAttempts &&
        calculateAccuracy() < levelDownAccuracy
    }

    func levelUp() -> GradeLevel? {
        guard let nextLevel = gradeLevel.nextLevel() else { return nil }
        gradeLevel = nextLevel
        attemptsSinceLevelChange = 0
        isCelebrating = false
        return gradeLevel
    }

    func levelDown() -> GradeLevel? {
        guard let previousLevel = gradeLevel.previousLevel() else { return nil }
        gradeLevel = previousLevel
        attemptsSinceLevelChange = 0
        isCelebrating = false
        return gradeLevel
    }

    func updateWordList(_ words: [String]) {
        availableWords = words
        nextWord()
    }

    func finishCelebration() {
        isCelebrating = false
        nextWord()
    }

    func addLetter(_ letter: Letter) {
        typedLetters.append(letter)
    }

    func removeLastLetter() {
        if !typedLetters.isEmpty {
            typedLetters.removeLast()
        }
    }

    func clearTyped() {
        typedLetters.removeAll()
    }
}
