fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> {
        buf_try_get_impl!(self, u64::from_le_bytes)
    }