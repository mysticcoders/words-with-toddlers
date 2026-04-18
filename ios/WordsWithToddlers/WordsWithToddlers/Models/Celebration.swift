import Foundation

struct Celebration {
    let startTime: Date
    let duration: TimeInterval = 1.5

    init() {
        self.startTime = Date()
    }

    func progress() -> Double {
        let elapsed = Date().timeIntervalSince(startTime)
        return min(elapsed / duration, 1.0)
    }

    func isActive() -> Bool {
        progress() < 1.0
    }

    func scaleFactor() -> Double {
        let p = progress()
        if p < 0.3 {
            return 1.0 + (p / 0.3) * 0.5
        } else if p < 0.7 {
            return 1.5
        } else {
            return 1.5 - ((p - 0.7) / 0.3) * 0.5
        }
    }

    func opacity() -> Double {
        let p = progress()
        if p < 0.8 {
            return 1.0
        } else {
            return 1.0 - ((p - 0.8) / 0.2)
        }
    }
}
