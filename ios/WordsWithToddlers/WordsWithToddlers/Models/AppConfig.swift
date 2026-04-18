import Foundation

struct AppConfig: Codable {
    var selectedSound: String
    var lastSelectedGrade: GradeLevel

    static let `default` = AppConfig(
        selectedSound: "Swoosh",
        lastSelectedGrade: .preK
    )
}
