fn try_get_i128(&mut self) -> Result<i128, TryGetError> {
        buf_try_get_impl!(self, i128::from_be_bytes)
    }