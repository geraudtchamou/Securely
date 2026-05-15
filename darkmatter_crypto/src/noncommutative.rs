pub struct NonCommutativeOp;

impl NonCommutativeOp {
    pub fn new() -> Self { Self }

    pub fn multiply(&self, a: &[f64], b: &[i64]) -> Vec<f64> {
        let len = a.len().min(b.len());
        let mut result = Vec::with_capacity(len);
        for i in 0..len {
            result.push(a[i] * b[i] as f64 - a[(i+1) % a.len()] * b[(i+2) % b.len()] as f64);
        }
        result
    }

    pub fn norm(&self, vec: &[f64]) -> f64 {
        vec.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
}
