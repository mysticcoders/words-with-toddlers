import Foundation
import Combine

class WordListLoader: ObservableObject {
    private var wordsByGrade: [GradeLevel: [String]] = [:]

    init() {
        loadAllWordLists()
    }

    private func loadAllWordLists() {
        wordsByGrade[.preK] = loadWords(from: "dolch_pre_primer")
        wordsByGrade[.kindergarten] = loadWords(from: "dolch_primer")

        var firstGrade = loadWords(from: "dolch_first")
        firstGrade.append(contentsOf: loadWords(from: "fry_001_100"))
        wordsByGrade[.first] = firstGrade

        var secondGrade = loadWords(from: "dolch_second")
        secondGrade.append(contentsOf: loadWords(from: "fry_101_200"))
        wordsByGrade[.second] = secondGrade

        var thirdGrade = loadWords(from: "dolch_third")
        thirdGrade.append(contentsOf: loadWords(from: "fry_201_300"))
        wordsByGrade[.third] = thirdGrade

        wordsByGrade[.fourth] = loadWords(from: "fry_301_400")

        var fifthGrade = loadWords(from: "fry_401_500")
        fifthGrade.append(contentsOf: loadWords(from: "fry_501_600"))
        wordsByGrade[.fifth] = fifthGrade

        var sixthGrade = loadWords(from: "fry_601_700")
        sixthGrade.append(contentsOf: loadWords(from: "fry_701_800"))
        sixthGrade.append(contentsOf: loadWords(from: "fry_801_900"))
        sixthGrade.append(contentsOf: loadWords(from: "fry_901_1000"))
        wordsByGrade[.sixth] = sixthGrade
    }

    private func loadWords(from filename: String) -> [String] {
        guard let path = Bundle.main.path(forResource: filename, ofType: "txt", inDirectory: "word_lists"),
              let contents = try? String(contentsOfFile: path, encoding: .utf8) else {
            print("Failed to load word list: \(filename).txt")
            return []
        }

        return contents
            .split(separator: "\n")
            .map { $0.trimmingCharacters(in: .whitespaces).lowercased() }
            .filter { !$0.isEmpty }
    }

    func getWords(for grade: GradeLevel) -> [String]? {
        wordsByGrade[grade]
    }

    func wordCount(for grade: GradeLevel) -> Int {
        wordsByGrade[grade]?.count ?? 0
    }
}
