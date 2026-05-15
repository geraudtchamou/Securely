pub struct HolonomicGate;

impl HolonomicGate {
    pub fn new() -> Self { Self }

    pub fn apply_loop(&self, data: &[f64]) -> Vec<f64> {
        data.iter().enumerate().map(|(i, &x)| {
            x * ((i as f64) * std::f64::consts::PI / 8.0).sin()
        }).collect()
    }

    pub fn phase(&self, geometric: &[f64]) -> f64 {
        geometric.iter().sum::<f64>() / geometric.len() as f64
    }
}
