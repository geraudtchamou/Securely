pub struct KawasakiCA;

impl KawasakiCA {
    pub fn new() -> Self { Self }

    pub fn evolve(&self, plasma: &[f64]) -> Vec<f64> {
        let mut result = plasma.to_vec();
        for i in (0..result.len()-1).step_by(2) {
            let avg = (result[i] + result[i+1]) / 2.0;
            result[i] = avg;
            result[i+1] = avg;
        }
        result
    }

    pub fn conserved(&self, data: &[f64]) -> f64 {
        data.iter().sum::<f64>()
    }
}
