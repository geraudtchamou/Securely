pub struct GravitationalLens {
    gravitational_constant: f64,
}

impl GravitationalLens {
    pub fn new() -> Self {
        Self {
            gravitational_constant: 6.674e-11,
        }
    }

    pub fn warp_data(&self, data: &[u8], mass: &f64) -> Vec<u8> {
        let mut warped = Vec::with_capacity(data.len());
        
        for (i, &byte) in data.iter().enumerate() {
            // Simulate gravitational lensing: deflection angle α = 4GM/(c²b)
            let impact_parameter = (i as f64 + 1.0) * 1000.0;
            let c_squared = 299792458.0f64.powi(2);
            let deflection = (4.0 * self.gravitational_constant * mass.abs()) / (c_squared * impact_parameter);
            
            // Apply warping to byte value
            let warp_factor = 1.0 + deflection * 1e20; // Scale up for visibility
            let warped_byte = ((byte as f64 * warp_factor) % 256.0) as u8;
            warped.push(warped_byte);
        }
        
        warped
    }

    pub fn einstein_ring_radius(&self, mass: f64, distance: f64) -> f64 {
        // θ_E = sqrt(4GM/c² * D_ls/(D_l * D_s))
        let c_squared = 299792458.0f64.powi(2);
        (4.0 * self.gravitational_constant * mass / c_squared * distance).sqrt()
    }
}
