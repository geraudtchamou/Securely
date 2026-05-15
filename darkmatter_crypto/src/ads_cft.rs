pub struct HolographicCipher;

impl HolographicCipher {
    pub fn new() -> Self { Self }

    pub fn project_to_boundary(&self, bulk: &[f64]) -> Vec<u8> {
        bulk.chunks(3).map(|chunk| {
            ((chunk.iter().sum::<f64>() / chunk.len() as f64).abs() % 256.0) as u8
        }).collect()
    }
}
