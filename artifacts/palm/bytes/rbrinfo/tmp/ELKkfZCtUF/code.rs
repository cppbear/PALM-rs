fn try_get_uint(&mut self, nbytes: usize) -> Result<u64, TryGetError> {
        buf_try_get_impl!(be => self, u64, nbytes);
    }