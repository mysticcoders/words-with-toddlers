import Foundation
import Combine

class DictionaryService: ObservableObject {
    private var validWords: Set<String> = []

    init() {
        loadCuratedWords()
    }

    private func loadCuratedWords() {
        let wordLoader = WordListLoader()
        for grade in GradeLevel.allCases {
            if let words = wordLoader.getWords(for: grade) {
                validWords.formUnion(words)
            }
        }
    }

    func isValidWord(_ word: String) -> Bool {
        validWords.contains(word.lowercased())
    }
}
