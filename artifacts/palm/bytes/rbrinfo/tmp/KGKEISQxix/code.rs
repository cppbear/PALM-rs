fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError> {
        Ok(f64::from_bits(self.try_get_u64_ne()?))
    }