fn try_get_i64_le(&mut self) -> Result<i64, TryGetError> {
        buf_try_get_impl!(self, i64::from_le_bytes)
    }