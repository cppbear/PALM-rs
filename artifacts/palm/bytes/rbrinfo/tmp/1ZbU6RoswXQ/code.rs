fn try_get_uint_le(&mut self, nbytes: usize) -> Result<u64, TryGetError> {
        buf_try_get_impl!(le => self, u64, nbytes);
    }