fn try_get_i128_le(&mut self) -> Result<i128, TryGetError> {
        buf_try_get_impl!(self, i128::from_le_bytes)
    }