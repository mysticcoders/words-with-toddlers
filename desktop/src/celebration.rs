use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Celebration {
    start_time: Instant,
    duration: Duration,
}

impl Celebration {
    pub fn new() -> Self {
        Celebration {
            start_time: Instant::now(),
            duration: Duration::from_millis(1500),
        }
    }

    pub fn is_active(&self) -> bool {
        self.start_time.elapsed() < self.duration
    }

    pub fn progress(&self) -> f32 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let total = self.duration.as_secs_f32();
        (elapsed / total).min(1.0)
    }

    pub fn scale_factor(&self) -> f32 {
        let progress = self.progress();
        if progress < 0.3 {
            1.0 + (progress / 0.3) * 0.5
        } else if progress < 0.7 {
            1.5
        } else {
            1.5 - ((progress - 0.7) / 0.3) * 0.5
        }
    }

    pub fn opacity(&self) -> f32 {
        let progress = self.progress();
        if progress < 0.8 {
            1.0
        } else {
            1.0 - ((progress - 0.8) / 0.2)
        }
    }
}

impl Default for Celebration {
    fn default() -> Self {
        Self::new()
    }
}
