fn try_get_u16(&mut self) -> Result<u16, TryGetError> {
        buf_try_get_impl!(self, u16::from_be_bytes)
    }