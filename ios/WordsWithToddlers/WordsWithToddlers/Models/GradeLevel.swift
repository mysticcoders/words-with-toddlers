import Foundation

enum GradeLevel: String, Codable, CaseIterable {
    case preK = "PreK"
    case kindergarten = "Kindergarten"
    case first = "First"
    case second = "Second"
    case third = "Third"
    case fourth = "Fourth"
    case fifth = "Fifth"
    case sixth = "Sixth"

    var displayName: String {
        switch self {
        case .preK: return "Pre-K"
        case .kindergarten: return "Kindergarten"
        case .first: return "1st Grade"
        case .second: return "2nd Grade"
        case .third: return "3rd Grade"
        case .fourth: return "4th Grade"
        case .fifth: return "5th Grade"
        case .sixth: return "6th Grade"
        }
    }

    var shortName: String {
        switch self {
        case .preK: return "Pre-K"
        case .kindergarten: return "K"
        case .first: return "1st"
        case .second: return "2nd"
        case .third: return "3rd"
        case .fourth: return "4th"
        case .fifth: return "5th"
        case .sixth: return "6th"
        }
    }

    func nextLevel() -> GradeLevel? {
        let levels = GradeLevel.allCases
        guard let currentIndex = levels.firstIndex(of: self),
              currentIndex < levels.count - 1 else {
            return nil
        }
        return levels[currentIndex + 1]
    }

    func previousLevel() -> GradeLevel? {
        let levels = GradeLevel.allCases
        guard let currentIndex = levels.firstIndex(of: self),
              currentIndex > 0 else {
            return nil
        }
        return levels[currentIndex - 1]
    }
}
