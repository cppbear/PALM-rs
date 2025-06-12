pub fn f64(&mut self) -> f64 {
        let b = 64;
        let f = core::f64::MANTISSA_DIGITS - 1;
        f64::from_bits((1 << (b - 2)) - (1 << f) + (self.u64(..) >> (b - f))) - 1.0
    }