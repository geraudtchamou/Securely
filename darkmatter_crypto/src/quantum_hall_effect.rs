pub struct QHETopology;

impl QHETopology {
    pub fn new() -> Self { Self }

    pub fn quantize(&self, pattern: &[f64]) -> Vec<f64> {
        pattern.iter().map(|&x| ((x * 10.0).round() / 10.0)).collect()
    }

    pub fn conductance(&self, hall: &[f64]) -> f64 {
        hall.len() as f64 * 0.0001
    }
}
