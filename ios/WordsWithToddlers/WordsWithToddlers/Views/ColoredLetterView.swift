import SwiftUI

struct ColoredLetterView: View {
    let letter: Letter
    let size: CGFloat

    var body: some View {
        Text(String(letter.character))
            .font(.system(size: size, weight: .bold))
            .foregroundColor(letter.color)
    }
}
