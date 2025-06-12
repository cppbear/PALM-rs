fn try_get_i16(&mut self) -> Result<i16, TryGetError> {
        buf_try_get_impl!(self, i16::from_be_bytes)
    }