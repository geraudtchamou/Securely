use crate::schumann::SchumannEngine;

pub struct MagneticField {
    earth_radius: f64,
    magnetic_moment: f64,
}

impl MagneticField {
    pub fn new() -> Self {
        Self {
            earth_radius: 6371.0, // km
            magnetic_moment: 7.94e22, // A·m²
        }
    }

    pub fn dipole_field(&self, frequencies: &[f64]) -> f64 {
        // Calculate magnetic field strength using dipole equation
        let mut total_field = 0.0;
        
        for &freq in frequencies {
            // B = (μ₀/4π) * (M/r³) * sqrt(1 + 3sin²θ)
            // Simplified model using frequency as angular parameter
            let theta = freq * 0.001;
            let sin_theta = theta.sin();
            let field_strength = (1.0 + 3.0 * sin_theta.powi(2)).sqrt();
            total_field += field_strength * freq.abs().ln();
        }
        
        total_field / frequencies.len() as f64
    }

    pub fn magnetopause_standoff(&self, solar_wind_pressure: f64) -> f64 {
        // r_mp = (B₀² / (2μ₀P_sw))^(1/6) * R_E
        let b0_squared = self.magnetic_moment.powi(2);
        let pressure_term = 2.0 * 4.0 * std::f64::consts::PI * solar_wind_pressure;
        (b0_squared / pressure_term).powf(1.0/6.0) * self.earth_radius
    }
}
