fn get_f64(&mut self) -> f64 {
        f64::from_bits(self.get_u64())
    }