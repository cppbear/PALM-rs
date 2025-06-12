fn try_get_u16_le(&mut self) -> Result<u16, TryGetError> {
        buf_try_get_impl!(self, u16::from_le_bytes)
    }