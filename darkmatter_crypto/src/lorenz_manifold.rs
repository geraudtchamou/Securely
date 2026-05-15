pub struct Lorenz3D;

impl Lorenz3D {
    pub fn new() -> Self { Self }

    pub fn evolve(&self, data: &[f64]) -> Vec<f64> {
        let sigma = 10.0;
        let rho = 28.0;
        let beta = 8.0/3.0;
        let mut result = Vec::with_capacity(data.len());
        for i in 0..data.len()/3 {
            let x = data[i*3];
            let y = if i*3+1 < data.len() { data[i*3+1] } else { 0.0 };
            let z = if i*3+2 < data.len() { data[i*3+2] } else { 0.0 };
            let dx = sigma * (y - x);
            let dy = x * (rho - z) - y;
            let dz = x * y - beta * z;
            result.push(x + dx * 0.01);
            result.push(y + dy * 0.01);
            result.push(z + dz * 0.01);
        }
        result
    }

    pub fn distance(&self, chaotic: &[f64]) -> f64 {
        chaotic.iter().map(|&x| x * x).sum::<f64>().sqrt()
    }
}
