pub struct DarkMatterHalo;

impl DarkMatterHalo {
    pub fn new() -> Self {
        Self
    }
    
    pub fn nfw_profile(&self, radius: f64, scale_radius: f64) -> f64 {
        // Navarro-Frenk-White profile: ρ(r) = ρ₀ / [(r/r_s)(1 + r/r_s)²]
        let x = radius / scale_radius;
        if x <= 0.0 {
            return 1.0;
        }
        1.0 / (x * (1.0 + x).powi(2))
    }
}
