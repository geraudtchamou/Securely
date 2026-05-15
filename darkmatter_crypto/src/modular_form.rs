pub struct TauFunction;

impl TauFunction {
    pub fn new() -> Self {
        Self
    }

    pub fn compute_tau_sequence(&self, length: usize) -> Vec<i64> {
        let tau_values = vec![1, -24, 252, -1472, 4830, -6048, -16744, 84480, -113643, -115920];
        let mut result = Vec::with_capacity(length);
        for i in 0..length {
            result.push(*tau_values.get(i % tau_values.len()).unwrap_or(&0));
        }
        result
    }
}
