use std::f64::consts::{PI, E};

pub struct SchumannEngine {
    base_frequency: f64,
    golden_ratio: f64,
}

impl SchumannEngine {
    pub fn new() -> Self {
        Self {
            base_frequency: 7.83,
            golden_ratio: (1.0 + 5.0.sqrt()) / 2.0,
        }
    }

    pub fn generate_frequencies(&self, count: usize) -> Vec<f64> {
        let mut freqs = Vec::with_capacity(count);
        let max_freq = 64.0f64.powi(7); // 64^7
        
        for i in 0..count {
            // Schumann resonance harmonics with Pi and Golden Ratio scaling
            let harmonic = (i + 1) as f64;
            let base = self.base_frequency * harmonic;
            
            // Apply Pi scaling
            let pi_scaled = base * PI.powf(i as f64 * 0.1);
            
            // Apply Golden Ratio scaling to reach galactic scales
            let phi_scaled = pi_scaled * self.golden_ratio.powf(i as f64 * 0.5);
            
            // Clamp to max frequency
            let clamped = phi_scaled.min(max_freq);
            
            freqs.push(clamped);
        }
        
        freqs
    }

    pub fn map_to_stellar_pattern(&self, freq: f64) -> f64 {
        // Map frequency to estimated number of stars using Pi and Phi
        let stellar_count = (freq * PI * self.golden_ratio).ln() * 1e6;
        stellar_count
    }
}
