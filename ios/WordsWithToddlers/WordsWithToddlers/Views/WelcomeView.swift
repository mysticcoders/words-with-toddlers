import SwiftUI

struct WelcomeView: View {
    let onStartVisualChallenge: () -> Void
    let onStartAudioChallenge: () -> Void
    let onStartDiscovery: () -> Void
    let onOpenSettings: () -> Void

    var body: some View {
        VStack(spacing: 40) {
            Spacer()

            Text("Words with Toddlers")
                .font(.system(size: 60, weight: .bold))
                .foregroundColor(.blue)
                .multilineTextAlignment(.center)

            Text("Choose a Mode")
                .font(.system(size: 30))
                .foregroundColor(.secondary)

            VStack(spacing: 20) {
                Button(action: onStartVisualChallenge) {
                    HStack {
                        Text("👁️")
                            .font(.system(size: 40))
                        Text("See Words")
                            .font(.system(size: 30, weight: .medium))
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.blue)
                    .foregroundColor(.white)
                    .cornerRadius(15)
                }

                Button(action: onStartAudioChallenge) {
                    HStack {
                        Text("🔊")
                            .font(.system(size: 40))
                        Text("Hear Words")
                            .font(.system(size: 30, weight: .medium))
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.green)
                    .foregroundColor(.white)
                    .cornerRadius(15)
                }

                Button(action: onStartDiscovery) {
                    HStack {
                        Text("✏️")
                            .font(.system(size: 40))
                        Text("Discovery")
                            .font(.system(size: 30, weight: .medium))
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.purple)
                    .foregroundColor(.white)
                    .cornerRadius(15)
                }

                Button(action: onOpenSettings) {
                    HStack {
                        Text("⚙️")
                            .font(.system(size: 40))
                        Text("Settings")
                            .font(.system(size: 30, weight: .medium))
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.gray)
                    .foregroundColor(.white)
                    .cornerRadius(15)
                }
            }
            .padding(.horizontal, 40)

            Spacer()
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(Color(UIColor.systemBackground))
    }
}
