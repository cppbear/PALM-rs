fn try_get_u32_le(&mut self) -> Result<u32, TryGetError> {
        buf_try_get_impl!(self, u32::from_le_bytes)
    }