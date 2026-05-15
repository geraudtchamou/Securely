pub struct TuringPattern;

impl TuringPattern {
    pub fn new() -> Self { Self }

    pub fn generate(&self, data: &[f64]) -> Vec<f64> {
        let mut result = data.to_vec();
        for _ in 0..5 {
            let mut new_result = vec![0.0; result.len()];
            for i in 0..result.len() {
                let left = if i == 0 { result[result.len()-1] } else { result[i-1] };
                let right = if i == result.len()-1 { result[0] } else { result[i+1] };
                new_result[i] = result[i] + 0.1 * (left - 2.0 * result[i] + right);
            }
            result = new_result;
        }
        result
    }

    pub fn count_spots(&self, pattern: &[f64]) -> usize {
        pattern.windows(2).filter(|w| w[0] > 0.0 && w[1] < 0.0).count()
    }
}
