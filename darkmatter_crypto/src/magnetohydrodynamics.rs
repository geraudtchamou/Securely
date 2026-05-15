pub struct PlasmaFlow;

impl PlasmaFlow {
    pub fn new() -> Self { Self }

    pub fn flow(&self, foam: &[f64]) -> Vec<f64> {
        let mut result = foam.to_vec();
        for i in 1..result.len() {
            result[i] = (result[i] + result[i-1]) / 2.0;
        }
        result
    }

    pub fn alfven_speed(&self, plasma: &[f64]) -> f64 {
        plasma.iter().map(|&x| x.abs()).sum::<f64>() / plasma.len() as f64
    }
}
