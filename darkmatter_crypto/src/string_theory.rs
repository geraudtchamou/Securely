pub struct StringVibrator;

impl StringVibrator {
    pub fn new() -> Self { Self }

    pub fn vibrate(&self, data: &[u8]) -> Vec<f64> {
        data.iter().enumerate().map(|(i, &x)| {
            (x as f64) * ((i as f64) * 0.1).sin()
        }).collect()
    }
}
