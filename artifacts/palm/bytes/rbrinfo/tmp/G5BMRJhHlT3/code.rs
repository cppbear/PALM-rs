fn try_get_i32(&mut self) -> Result<i32, TryGetError> {
        buf_try_get_impl!(self, i32::from_be_bytes)
    }