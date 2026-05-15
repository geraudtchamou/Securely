pub struct RamanujanSystem;

impl RamanujanSystem {
    pub fn new() -> Self { Self }

    pub fn mock_theta_permutation(&self, length: usize) -> Vec<usize> {
        let mut perm: Vec<usize> = (0..length).collect();
        for i in (1..length).rev() {
            let j = (i * 7 + 3) % (i + 1);
            perm.swap(i, j);
        }
        perm
    }

    pub fn partition_lattice(&self, data: &[u8]) -> Vec<u8> {
        data.iter().enumerate().map(|(i, &x)| x ^ ((i * 17) % 256) as u8).collect()
    }
}
