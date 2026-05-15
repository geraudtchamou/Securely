pub struct HagedornEngine;

impl HagedornEngine {
    pub fn new() -> Self { Self }

    pub fn expand(&self, data: &[f64]) -> Vec<f64> {
        let mut result = Vec::new();
        for &x in data {
            result.push(x);
            if x.abs() > 0.3 {
                result.push(x * 0.7);
            }
        }
        result
    }

    pub fn temperature_ratio(&self, expanded: &[f64]) -> f64 {
        expanded.len() as f64 / (expanded.iter().filter(|&&x| x.abs() > 0.5).count() as f64 + 1.0)
    }
}
