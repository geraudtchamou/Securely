pub struct NBodySimulator {
    g: f64,
}

impl NBodySimulator {
    pub fn new() -> Self {
        Self { g: 6.674e-11 }
    }

    pub fn simulate(&self, data: &[u8], steps: usize) -> Vec<u8> {
        let mut result = data.to_vec();
        
        for _ in 0..steps {
            let mut new_result = vec![0u8; result.len()];
            
            for i in 0..result.len() {
                let mut force = 0.0;
                
                for j in 0..result.len() {
                    if i != j {
                        let r = ((i - j).abs() as f64 + 1.0) * 1000.0;
                        let m1 = result[i] as f64;
                        let m2 = result[j] as f64;
                        force += self.g * m1 * m2 / (r * r);
                    }
                }
                
                new_result[i] = ((result[i] as f64 + force * 1e15) % 256.0) as u8;
            }
            
            result = new_result;
        }
        
        result
    }
}
