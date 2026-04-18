import Foundation
import Combine

class ConfigService: ObservableObject {
    private let userDefaults = UserDefaults.standard
    private let configKey = "appConfig"

    func loadConfig() -> AppConfig {
        guard let data = userDefaults.data(forKey: configKey),
              let config = try? JSONDecoder().decode(AppConfig.self, from: data) else {
            return .default
        }
        return config
    }

    func saveConfig(_ config: AppConfig) {
        guard let data = try? JSONEncoder().encode(config) else { return }
        userDefaults.set(data, forKey: configKey)
    }
}
