pub struct AnyonBraider;

impl AnyonBraider {
    pub fn new() -> Self { Self }

    pub fn braid(&self, data: &[f64]) -> Vec<f64> {
        let mut result = data.to_vec();
        for i in (0..result.len()-1).step_by(2) {
            result.swap(i, i+1);
        }
        result
    }

    pub fn winding_number(&self, data: &[f64]) -> usize {
        data.windows(2).filter(|w| w[0] > w[1]).count()
    }
}
