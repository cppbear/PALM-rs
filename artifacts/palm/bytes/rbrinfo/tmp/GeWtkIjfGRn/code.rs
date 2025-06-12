fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError> {
        buf_try_get_impl!(self, i16::from_ne_bytes)
    }