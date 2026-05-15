pub struct FluidTurbulence;

impl FluidTurbulence {
    pub fn new() -> Self { Self }

    pub fn mix(&self, data: &[f64]) -> Vec<f64> {
        let mut result = data.to_vec();
        for i in 1..result.len()-1 {
            result[i] = (result[i-1] + result[i] * 2.0 + result[i+1]) / 4.0;
        }
        result
    }

    pub fn reynolds(&self, flow: &[f64]) -> f64 {
        flow.iter().map(|&x| x.abs()).sum::<f64>() * 1000.0
    }
}
