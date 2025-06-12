fn get_f32_ne(&mut self) -> f32 {
        f32::from_bits(self.get_u32_ne())
    }