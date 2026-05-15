pub struct VacuumBirefringence;

impl VacuumBirefringence {
    pub fn new() -> Self { Self }

    pub fn split_polarization(&self, defects: &[usize]) -> (Vec<f64>, Vec<f64>, f64) {
        let mode1: Vec<f64> = defects.iter().map(|&x| (x as f64).sin()).collect();
        let mode2: Vec<f64> = defects.iter().map(|&x| (x as f64).cos()).collect();
        let phase_shift = (mode1.len() as f64) * 0.01;
        (mode1, mode2, phase_shift)
    }
}
