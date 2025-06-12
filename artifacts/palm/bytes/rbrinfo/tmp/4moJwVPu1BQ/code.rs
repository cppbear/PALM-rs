fn try_get_i64(&mut self) -> Result<i64, TryGetError> {
        buf_try_get_impl!(self, i64::from_be_bytes)
    }