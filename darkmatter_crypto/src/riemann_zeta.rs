pub struct ZetaGenerator;

impl ZetaGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_key_stream(&self, length: usize) -> Vec<f64> {
        let mut keys = Vec::with_capacity(length);
        for n in 1..=length {
            let mut product = 1.0;
            for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29].iter() {
                product *= 1.0 / (1.0 - (*p as f64).powi(-2));
            }
            keys.push(product * (n as f64).sin());
        }
        keys
    }
}
