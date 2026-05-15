pub struct MaxwellDemon;

impl MaxwellDemon {
    pub fn new() -> Self { Self }

    pub fn sort_and_extract(&self, manifold: &[f64]) -> (Vec<u8>, f64) {
        let mut sorted = manifold.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let work = sorted.iter().sum::<f64>().abs();
        let bytes = sorted.iter().map(|&x| ((x * 127.0 + 128.0) % 256.0) as u8).collect();
        (bytes, work)
    }
}
