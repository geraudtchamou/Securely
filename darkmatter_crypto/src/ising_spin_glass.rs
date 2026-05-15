pub struct IsingModel;

impl IsingModel {
    pub fn new() -> Self { Self }

    pub fn simulate(&self, quasicryst: &[f64]) -> Vec<i8> {
        quasicryst.iter().map(|&x| if x > 0.0 { 1 } else { -1 }).collect()
    }

    pub fn magnetization(&self, spin_state: &[i8]) -> f64 {
        spin_state.iter().sum::<i8>() as f64 / spin_state.len() as f64
    }
}
