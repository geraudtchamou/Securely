//! Dark Matter Cryptographic System
//! Combines Schumann resonance, planetary magnetic dynamics, and hypercolor cryptography

use crate::schumann::{generate_schumann_frequencies, calculate_magnetic_field, calculate_magnetopause};
use crate::color_crypto::{HyperColor, encrypt, decrypt, calculate_gravitational_influence, HYPERCOLOR_DIMENSIONS};

pub struct DarkMatterCrypto {
    frequencies: Vec<f64>,
    current_key_index: usize,
}

impl DarkMatterCrypto {
    pub fn new() -> Self {
        let frequencies = generate_schumann_frequencies();
        DarkMatterCrypto {
            frequencies,
            current_key_index: 0,
        }
    }
    
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        encrypt(data, self.current_key_index)
    }
    
    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        decrypt(data, self.current_key_index)
    }
    
    pub fn rotate_key(&mut self) {
        self.current_key_index = (self.current_key_index + 1) % self.frequencies.len();
    }
    
    pub fn set_key_index(&mut self, index: usize) {
        self.current_key_index = index % self.frequencies.len();
    }
    
    pub fn current_frequency(&self) -> f64 {
        self.frequencies[self.current_key_index]
    }
    
    pub fn simulate_dark_matter_properties(&self, data: &[u8]) -> DarkMatterSimulation {
        let frequency = self.current_frequency();
        let magnetic_field = calculate_magnetic_field(frequency, 1.0);
        let solar_wind_pressure = 2.0e-9;
        let magnetopause = calculate_magnetopause(frequency, solar_wind_pressure);
        let key = HyperColor::generate_key(self.current_key_index);
        let darkness = key.darkness();
        let influences = calculate_gravitational_influence(data);
        
        DarkMatterSimulation {
            frequency,
            magnetic_field,
            magnetopause_distance: magnetopause,
            darkness_metric: darkness,
            gravitational_points: influences.len(),
            hypercolor_dimensions: HYPERCOLOR_DIMENSIONS,
        }
    }
    
    pub fn display_stats(&self) {
        println!("=== Dark Matter Crypto System ===");
        println!("Frequency range: 2 to 64^7 ({})", 64u64.pow(7));
        println!("Total frequencies generated: {}", self.frequencies.len());
        println!("Current key index: {}", self.current_key_index);
        println!("Current frequency: {:.6} Hz", self.current_frequency());
        println!("HyperColor dimensions: {}", HYPERCOLOR_DIMENSIONS);
        println!("\nMathematical constants used:");
        println!("  Pi (π): {:.10}", core::f64::consts::PI);
        println!("  Golden Ratio (φ): {:.10}", 1.618033988749895);
        println!("\nCelestial bound (64^7): {}", 64u64.pow(7));
    }
}

#[derive(Debug)]
pub struct DarkMatterSimulation {
    pub frequency: f64,
    pub magnetic_field: f64,
    pub magnetopause_distance: f64,
    pub darkness_metric: f64,
    pub gravitational_points: usize,
    pub hypercolor_dimensions: usize,
}

impl core::fmt::Display for DarkMatterSimulation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "=== Dark Matter Simulation ===")?;
        writeln!(f, "Schumann Frequency: {:.6} Hz", self.frequency)?;
        writeln!(f, "Magnetic Field Strength: {:.6e} T", self.magnetic_field)?;
        writeln!(f, "Magnetopause Distance: {:.2} m", self.magnetopause_distance)?;
        writeln!(f, "Darkness Metric (invisibility): {:.4}", self.darkness_metric)?;
        writeln!(f, "Gravitational Attraction Points: {}", self.gravitational_points)?;
        writeln!(f, "HyperColor Dimensions: {}", self.hypercolor_dimensions)?;
        writeln!(f, "\nProperties mimicked:")?;
        writeln!(f, "  ✓ Invisibility: Encrypted data appears as random noise")?;
        writeln!(f, "  ✓ Pervasiveness: Uses universal constants (π, φ)")?;
        writeln!(f, "  ✓ Gravitational Influence: Data creates attraction patterns")?;
        writeln!(f, "  ✓ Magnetic Dynamics: Planetary field simulation")?;
        Ok(())
    }
}

impl Default for DarkMatterCrypto {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_crypto_engine() {
        let mut crypto = DarkMatterCrypto::new();
        let data = b"Secret message about dark matter";
        
        let encrypted = crypto.encrypt(data);
        assert_ne!(data.to_vec(), encrypted);
        
        let decrypted = crypto.decrypt(&encrypted);
        assert_eq!(data.to_vec(), decrypted);
    }
    
    #[test]
    fn test_key_rotation() {
        let mut crypto = DarkMatterCrypto::new();
        let initial_freq = crypto.current_frequency();
        
        crypto.rotate_key();
        let new_freq = crypto.current_frequency();
        
        assert!((initial_freq - new_freq).abs() > 0.001 || crypto.current_key_index == 0);
    }
}
