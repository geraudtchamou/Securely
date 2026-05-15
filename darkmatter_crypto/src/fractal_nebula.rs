pub struct FractalDiffuser;

impl FractalDiffuser {
    pub fn new() -> Self {
        Self
    }

    pub fn diffuse(&self, data: &[u8], iterations: usize) -> Vec<u8> {
        let mut result = data.to_vec();
        for _ in 0..iterations {
            let mut new_result = vec![0u8; result.len()];
            for i in 0..result.len() {
                let left = if i == 0 { result[result.len()-1] } else { result[i-1] };
                let right = if i == result.len()-1 { result[0] } else { result[i+1] };
                new_result[i] = ((result[i] as u16 + left as u16 + right as u16) / 3) as u8;
            }
            result = new_result;
        }
        result
    }
}
