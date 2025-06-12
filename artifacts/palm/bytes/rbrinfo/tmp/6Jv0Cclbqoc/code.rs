fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError> {
        Ok(f32::from_bits(self.try_get_u32_ne()?))
    }