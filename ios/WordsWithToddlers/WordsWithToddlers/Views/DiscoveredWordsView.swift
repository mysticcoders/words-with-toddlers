import SwiftUI

struct DiscoveredWordsView: View {
    let words: [DiscoveredWord]

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 10) {
                ForEach(words) { word in
                    Text(word.text.uppercased())
                        .font(.system(size: 24, weight: .medium))
                        .foregroundColor(.green)
                }
            }
            .padding()
        }
    }
}
