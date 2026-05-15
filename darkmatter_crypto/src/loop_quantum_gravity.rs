pub struct LQGSpinFoam;

impl LQGSpinFoam {
    pub fn new() -> Self { Self }

    pub fn create_foam(&self, spin_state: &[i8]) -> Vec<f64> {
        spin_state.iter().map(|&s| (s as f64).abs() * std::f64::consts::PI).collect()
    }

    pub fn area(&self, foam: &[f64]) -> f64 {
        foam.iter().map(|&x| x * x).sum::<f64>().sqrt()
    }
}
