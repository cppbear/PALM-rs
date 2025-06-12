fn try_get_u32(&mut self) -> Result<u32, TryGetError> {
        buf_try_get_impl!(self, u32::from_be_bytes)
    }