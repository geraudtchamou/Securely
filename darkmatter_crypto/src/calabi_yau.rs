pub struct CalabiYauMapper;

impl CalabiYauMapper {
    pub fn new() -> Self {
        Self
    }

    pub fn map_to_6d(&self, data: &[u8]) -> Vec<f64> {
        let mut result = Vec::with_capacity(data.len() * 6);
        for (i, &byte) in data.iter().enumerate() {
            for dim in 0..6 {
                let phase = (i as f64 + dim as f64) * std::f64::consts::PI / 3.0;
                result.push((byte as f64) * phase.sin());
            }
        }
        result
    }

    pub fn volume(&self, coords: &[f64]) -> f64 {
        coords.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
}
