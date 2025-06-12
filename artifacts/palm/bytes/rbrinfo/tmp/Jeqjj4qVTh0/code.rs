fn get_f32(&mut self) -> f32 {
        f32::from_bits(self.get_u32())
    }