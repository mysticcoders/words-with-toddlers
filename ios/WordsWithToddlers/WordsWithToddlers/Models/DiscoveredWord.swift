import SwiftUI

struct DiscoveredWord: Identifiable, Codable {
    let id = UUID()
    let text: String
    let timestamp: Date

    init(text: String) {
        self.text = text
        self.timestamp = Date()
    }
}
