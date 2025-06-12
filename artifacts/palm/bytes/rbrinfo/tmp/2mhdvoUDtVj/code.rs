fn get_f64_ne(&mut self) -> f64 {
        f64::from_bits(self.get_u64_ne())
    }