fn get_f32_le(&mut self) -> f32 {
        f32::from_bits(self.get_u32_le())
    }