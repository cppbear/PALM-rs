fn try_get_int(&mut self, nbytes: usize) -> Result<i64, TryGetError> {
        buf_try_get_impl!(be => self, i64, nbytes);
    }