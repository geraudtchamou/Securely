pub struct TuraevViro;

impl TuraevViro {
    pub fn new() -> Self { Self }

    pub fn compute_invariant(&self, data: &[u8]) -> f64 {
        data.iter().map(|&x| (x as f64).sin().abs()).sum()
    }
}
