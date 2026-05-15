//! HyperColor Cryptography Module
//! Multi-dimensional color space encryption beyond RGB

use crate::schumann::{frequency_to_hyper_vector, generate_schumann_frequencies};

pub const HYPERCOLOR_DIMENSIONS: usize = 24;

#[derive(Clone, Debug)]
pub struct HyperColor {
    pub components: Vec<f64>,
}

impl HyperColor {
    pub fn from_frequency(freq: f64) -> Self {
        let components = frequency_to_hyper_vector(freq, HYPERCOLOR_DIMENSIONS);
        HyperColor { components }
    }
    
    pub fn generate_key(index: usize) -> Self {
        let freqs = generate_schumann_frequencies();
        let freq = freqs[index % freqs.len()];
        HyperColor::from_frequency(freq)
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for comp in &self.components {
            let scaled = ((comp.abs().sin() + 1.0) / 2.0 * 255.0) as u8;
            bytes.push(scaled);
        }
        bytes
    }
    
    pub fn darkness(&self) -> f64 {
        let sum: f64 = self.components.iter().map(|c| c.abs()).sum();
        let avg = sum / self.components.len() as f64;
        (avg.sin().abs() + 0.5).min(1.0)
    }
}

pub fn encrypt(data: &[u8], key_index: usize) -> Vec<u8> {
    let key = HyperColor::generate_key(key_index);
    let key_bytes = key.to_bytes();
    
    data.iter()
        .enumerate()
        .map(|(i, &byte)| {
            let key_byte = key_bytes[i % key_bytes.len()];
            byte ^ key_byte
        })
        .collect()
}

pub fn decrypt(data: &[u8], key_index: usize) -> Vec<u8> {
    encrypt(data, key_index)
}

pub fn calculate_gravitational_influence(data: &[u8]) -> Vec<(f64, f64, f64)> {
    let mut influences = Vec::new();
    
    for chunk in data.chunks(3) {
        if chunk.len() >= 3 {
            let x = chunk[0] as f64 / 255.0;
            let y = chunk[1] as f64 / 255.0;
            let z = chunk[2] as f64 / 255.0;
            
            let mass = (x * y * z).sqrt();
            influences.push((x * mass, y * mass, z * mass));
        }
    }
    
    influences
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hypercolor_creation() {
        let hc = HyperColor::from_frequency(7.83);
        assert_eq!(hc.components.len(), HYPERCOLOR_DIMENSIONS);
    }
    
    #[test]
    fn test_encryption_decryption() {
        let data = b"Hello, Dark Matter!";
        let encrypted = encrypt(data, 0);
        let decrypted = decrypt(&encrypted, 0);
        assert_eq!(data.to_vec(), decrypted);
    }
}
