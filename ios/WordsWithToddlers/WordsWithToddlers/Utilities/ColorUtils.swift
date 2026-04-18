import SwiftUI

extension Color {
    static func randomLetterColor() -> Color {
        let hue = Double.random(in: 0...1)
        return Color(hue: hue, saturation: 0.8, brightness: 0.6)
    }
}
