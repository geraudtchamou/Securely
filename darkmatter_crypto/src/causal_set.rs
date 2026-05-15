pub struct CausalSet;

impl CausalSet {
    pub fn new() -> Self { Self }

    pub fn build_hasse(&self, data: &[f64]) -> String {
        format!("causal_relations_{}", data.len())
    }
}
