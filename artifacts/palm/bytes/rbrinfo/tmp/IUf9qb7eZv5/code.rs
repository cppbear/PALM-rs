fn try_get_f32(&mut self) -> Result<f32, TryGetError> {
        Ok(f32::from_bits(self.try_get_u32()?))
    }