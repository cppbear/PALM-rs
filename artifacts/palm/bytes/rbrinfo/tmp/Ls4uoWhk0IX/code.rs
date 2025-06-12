fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError> {
        buf_try_get_impl!(self, i32::from_ne_bytes)
    }