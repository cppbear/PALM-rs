fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError> {
        buf_try_get_impl!(self, u128::from_ne_bytes)
    }