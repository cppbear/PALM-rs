fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError> {
        buf_try_get_impl!(self, u64::from_ne_bytes)
    }