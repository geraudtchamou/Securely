pub struct HyperbolicWarper;

impl HyperbolicWarper {
    pub fn new() -> Self { Self }

    pub fn warp(&self, data: &[f64]) -> Vec<f64> {
        data.iter().map(|&x| x.tanh()).collect()
    }

    pub fn curvature(&self, data: &[f64]) -> f64 {
        data.iter().map(|&x| -x * x).sum::<f64>() / data.len() as f64
    }
}
