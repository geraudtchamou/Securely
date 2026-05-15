pub struct SonicBlackHole;

impl SonicBlackHole {
    pub fn new() -> Self { Self }

    pub fn create_horizon(&self, data: &[u8]) -> String {
        format!("horizon_events_{}", data.len())
    }
}
