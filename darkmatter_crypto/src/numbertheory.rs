pub struct PrimeGap;

impl PrimeGap {
    pub fn new() -> Self {
        Self
    }
    
    pub fn nth_prime(&self, n: usize) -> u64 {
        let mut count = 0;
        let mut num = 2;
        
        while count < n {
            if self.is_prime(num) {
                count += 1;
            }
            if count < n {
                num += 1;
            }
        }
        
        num
    }
    
    fn is_prime(&self, n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        
        let sqrt_n = (n as f64).sqrt() as u64;
        for i in (3..=sqrt_n).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        
        true
    }
}
