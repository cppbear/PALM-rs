pub fn f32(&mut self) -> f32 {
        let b = 32;
        let f = core::f32::MANTISSA_DIGITS - 1;
        f32::from_bits((1 << (b - 2)) - (1 << f) + (self.u32(..) >> (b - f))) - 1.0
    }