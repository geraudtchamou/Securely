pub struct FisherMetric;

impl FisherMetric {
    pub fn new() -> Self { Self }

    pub fn embed(&self, data: &[f64]) -> Vec<f64> {
        data.iter().map(|&x| x.abs().ln()).collect()
    }

    pub fn trace(&self, manifold: &[f64]) -> f64 {
        manifold.iter().map(|&x| x * x).sum()
    }
}
