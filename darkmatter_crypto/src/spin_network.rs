pub struct SpinNetwork;

impl SpinNetwork {
    pub fn new() -> Self { Self }

    pub fn evolve(&self, nodes: &[u8], steps: usize) -> Vec<u8> {
        let mut result = nodes.to_vec();
        for _ in 0..steps {
            let mut new_result = vec![0u8; result.len()];
            for i in 0..result.len() {
                let n1 = if i == 0 { result[result.len()-1] as u16 } else { result[i-1] as u16 };
                let n2 = if i == result.len()-1 { result[0] as u16 } else { result[i+1] as u16 };
                new_result[i] = ((result[i] as u16 + n1 + n2) / 3) as u8;
            }
            result = new_result;
        }
        result
    }
}
