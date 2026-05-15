pub struct PulsarTimer {
    period: f64,
}

impl PulsarTimer {
    pub fn new() -> Self {
        Self { period: 1.337 }
    }
    
    pub fn pulse(&self, time: f64) -> f64 {
        (time * 2.0 * std::f64::consts::PI / self.period).sin()
    }
}
