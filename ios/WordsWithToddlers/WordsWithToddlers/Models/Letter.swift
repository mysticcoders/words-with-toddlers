import SwiftUI

struct Letter: Identifiable, Equatable {
    let id = UUID()
    let character: Character
    let color: Color

    static func == (lhs: Letter, rhs: Letter) -> Bool {
        lhs.id == rhs.id
    }
}
