pub struct QuasiMapper;

impl QuasiMapper {
    pub fn new() -> Self { Self }

    pub fn map(&self, shower: &[f64]) -> Vec<f64> {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        shower.iter().enumerate().map(|(i, &x)| x * phi.powi(i as i32 % 5)).collect()
    }

    pub fn order(&self, quasicryst: &[f64]) -> f64 {
        quasicryst.iter().map(|&x| x.abs()).sum::<f64>() / quasicryst.len() as f64
    }
}
