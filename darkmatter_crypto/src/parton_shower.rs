pub struct PartonCascade;

impl PartonCascade {
    pub fn new() -> Self { Self }

    pub fn cascade(&self, folded: &[f64]) -> Vec<f64> {
        let mut result = Vec::new();
        for &x in folded {
            result.push(x);
            if x.abs() > 0.5 {
                result.push(x * 0.5);
                result.push(x * 0.25);
            }
        }
        result
    }
}
