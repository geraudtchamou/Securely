pub struct KerrGeometry;

impl KerrGeometry {
    pub fn new() -> Self { Self }

    pub fn frame_drag(&self, data: &[f64]) -> Vec<f64> {
        data.iter().enumerate().map(|(i, &x)| {
            x * (1.0 + (i as f64) * 0.01).sin()
        }).collect()
    }

    pub fn drag_angle(&self, dragged: &[f64]) -> f64 {
        dragged.iter().sum::<f64>().abs() * 0.001
    }
}
