pub struct SurfaceCode;

impl SurfaceCode {
    pub fn new() -> Self { Self }

    pub fn encode(&self, data: &[(f64, f64)]) -> Vec<u8> {
        data.iter().flat_map(|(x, y)| {
            vec![(x * 127.0 + 128.0) as u8, (y * 127.0 + 128.0) as u8]
        }).collect()
    }
}
