pub struct TimeCrystal;

impl TimeCrystal {
    pub fn new() -> Self { Self }

    pub fn oscillate(&self, data: &[u8]) -> Vec<f64> {
        data.iter().enumerate().map(|(i, &x)| {
            (x as f64) * ((i as f64) * std::f64::consts::PI / 4.0).cos()
        }).collect()
    }

    pub fn phase(&self, data: &[f64]) -> f64 {
        data.iter().sum::<f64>() / data.len() as f64
    }
}
