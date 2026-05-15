pub struct CosmicStringNet;

impl CosmicStringNet {
    pub fn new() -> Self { Self }

    pub fn create_defects(&self, data: &[u8]) -> Vec<usize> {
        data.iter().enumerate().filter(|(_, &x)| x > 128).map(|(i, _)| i).collect()
    }
}
