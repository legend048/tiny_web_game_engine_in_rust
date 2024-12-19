pub struct Timer {
    duration: f32,
    elapsed: f32,
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        self.elapsed += delta_time;
        if self.elapsed >= self.duration {
            self.elapsed = 0.0;
            return true;
        }
        false
    }
}
