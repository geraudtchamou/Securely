//! Dark Matter Cryptographic System
//! A color-based cryptographic system using Schumann frequencies,
//! Pi, Golden Ratio, and planetary magnetic dynamics

mod schumann;
mod color_crypto;
mod darkmatter;

use darkmatter::DarkMatterCrypto;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║     Dark Matter Cryptographic System                      ║");
    println!("║     Using Schumann Resonance & Celestial Mathematics      ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    let mut crypto = DarkMatterCrypto::new();
    crypto.display_stats();
    
    println!("\n=== Encryption Demonstration ===");
    let original_message = b"Dark matter comprises approximately 27% of the universe.";
    println!("Original message: {}", String::from_utf8_lossy(original_message));
    
    let encrypted = crypto.encrypt(original_message);
    println!("Encrypted (hex): {:02x?}", encrypted);
    
    let decrypted = crypto.decrypt(&encrypted);
    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted));
    
    assert_eq!(original_message.to_vec(), decrypted);
    println!("✓ Encryption/Decryption verified!\n");
    
    println!("=== Key Rotation Demonstration ===");
    let initial_freq = crypto.current_frequency();
    println!("Initial frequency: {:.6} Hz", initial_freq);
    
    crypto.rotate_key();
    let new_freq = crypto.current_frequency();
    println!("After rotation: {:.6} Hz", new_freq);
    
    let encrypted_new = crypto.encrypt(original_message);
    println!("New encrypted output differs: {}", encrypted != encrypted_new);
    
    crypto.set_key_index(0);
    let encrypted_original = crypto.encrypt(original_message);
    println!("Back to original key matches: {}", encrypted == encrypted_original);
    
    println!("\n");
    let simulation = crypto.simulate_dark_matter_properties(original_message);
    println!("{}", simulation);
    
    println!("\n=== Sample Schumann Frequencies (scaled by π and φ) ===");
    let freqs = schumann::generate_schumann_frequencies();
    for i in 0..5.min(freqs.len()) {
        println!("  Index {}: {:.6} Hz", i, freqs[i]);
    }
    println!("  ... ({} total frequencies)", freqs.len());
    
    println!("\n=== HyperColor Vector Sample ===");
    let sample_freq = 7.83;
    let hyper_vector = schumann::frequency_to_hyper_vector(sample_freq, 12);
    println!("Frequency {} mapped to {}D vector:", sample_freq, hyper_vector.len());
    for (i, val) in hyper_vector.iter().enumerate() {
        println!("  Dimension {}: {:.6}", i, val);
    }
    
    println!("\n=== Planetary Magnetic Field Simulation ===");
    let distances = [1.0, 5.0, 10.0, 30.0, 100.0];
    for dist in &distances {
        let field = schumann::calculate_magnetic_field(sample_freq, *dist);
        println!("  At {:.1} AU: {:.6e} Tesla", dist, field);
    }
    
    println!("\n=== Gravitational Influence Points ===");
    let influences = color_crypto::calculate_gravitational_influence(&encrypted);
    println!("Generated {} attraction points from encrypted data", influences.len());
    for (i, (x, y, z)) in influences.iter().take(3).enumerate() {
        println!("  Point {}: ({:.4}, {:.4}, {:.4})", i, x, y, z);
    }
    
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║  System demonstrates dark matter properties:              ║");
    println!("║  • Invisibility through multi-dimensional encryption      ║");
    println!("║  • Pervasiveness via universal constants (π, φ)           ║");
    println!("║  • Gravitational-like data attraction patterns            ║");
    println!("║  • Dynamic planetary magnetic field simulation            ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
}
