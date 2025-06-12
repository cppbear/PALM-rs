fn try_get_u128_le(&mut self) -> Result<u128, TryGetError> {
        buf_try_get_impl!(self, u128::from_le_bytes)
    }