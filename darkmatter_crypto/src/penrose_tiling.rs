pub struct PenroseTiler;

impl PenroseTiler {
    pub fn new() -> Self { Self }

    pub fn generate(&self, seed: &[u8], tiles: usize) -> Vec<(f64, f64)> {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let mut result = Vec::with_capacity(tiles);
        for i in 0..tiles {
            let angle = (i as f64) * 2.0 * std::f64::consts::PI / phi;
            result.push((angle.cos(), angle.sin()));
        }
        result
    }
}
