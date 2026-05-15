pub struct Rule110CA;

impl Rule110CA {
    pub fn new() -> Self { Self }

    pub fn step(&self, state: &[u8]) -> Vec<u8> {
        let mut result = vec![0u8; state.len()];
        for i in 0..state.len() {
            let left = if i == 0 { state[state.len()-1] } else { state[i-1] };
            let center = state[i];
            let right = if i == state.len()-1 { state[0] } else { state[i+1] };
            let pattern = ((left & 1) << 2) | ((center & 1) << 1) | (right & 1);
            result[i] = match pattern {
                0b111 => 0, 0b110 => 0, 0b101 => 0, 0b100 => 1,
                0b011 => 1, 0b010 => 1, 0b001 => 1, 0b000 => 0,
                _ => 0,
            };
        }
        result
    }
}
