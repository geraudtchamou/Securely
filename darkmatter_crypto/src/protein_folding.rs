pub struct ProteinFolder;

impl ProteinFolder {
    pub fn new() -> Self { Self }

    pub fn fold(&self, hall: &[f64]) -> Vec<f64> {
        let mut result = hall.to_vec();
        for i in 1..result.len()-1 {
            result[i] = (result[i-1] + result[i+1]) / 2.0;
        }
        result
    }

    pub fn rmsd(&self, folded: &[f64]) -> f64 {
        folded.iter().map(|&x| x * x).sum::<f64>().sqrt() / folded.len() as f64
    }
}
