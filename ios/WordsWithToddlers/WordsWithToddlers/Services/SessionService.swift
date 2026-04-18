import Foundation

class SessionService {
    private let fileManager = FileManager.default
    private let dateFormatter: DateFormatter = {
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd"
        return formatter
    }()

    private let timeFormatter: DateFormatter = {
        let formatter = DateFormatter()
        formatter.dateFormat = "HH-mm-ss"
        return formatter
    }()

    private func getSessionsDirectory() -> URL? {
        guard let documentsDir = fileManager.urls(for: .documentDirectory, in: .userDomainMask).first else {
            return nil
        }
        return documentsDir.appendingPathComponent("WordsWithToddlers/sessions")
    }

    func saveSession(_ session: Session) {
        guard let sessionsDir = getSessionsDirectory() else { return }

        let dateFolder = sessionsDir.appendingPathComponent(dateFormatter.string(from: session.timestamp))

        try? fileManager.createDirectory(at: dateFolder, withIntermediateDirectories: true)

        let filename = "session_\(timeFormatter.string(from: session.timestamp)).json"
        let fileURL = dateFolder.appendingPathComponent(filename)

        guard let data = try? JSONEncoder().encode(session) else { return }
        try? data.write(to: fileURL)
    }

    func loadRecentSessions(limit: Int = 10) -> [Session] {
        guard let sessionsDir = getSessionsDirectory() else { return [] }

        guard let contents = try? fileManager.contentsOfDirectory(at: sessionsDir, includingPropertiesForKeys: nil) else {
            return []
        }

        var sessions: [Session] = []

        for dateFolder in contents {
            guard let files = try? fileManager.contentsOfDirectory(at: dateFolder, includingPropertiesForKeys: nil) else {
                continue
            }

            for file in files where file.pathExtension == "json" {
                guard let data = try? Data(contentsOf: file),
                      let session = try? JSONDecoder().decode(Session.self, from: data) else {
                    continue
                }
                sessions.append(session)
            }
        }

        return Array(sessions.sorted { $0.timestamp > $1.timestamp }.prefix(limit))
    }
}
