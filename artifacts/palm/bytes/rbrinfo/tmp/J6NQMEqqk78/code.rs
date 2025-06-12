fn try_get_int_le(&mut self, nbytes: usize) -> Result<i64, TryGetError> {
        buf_try_get_impl!(le => self, i64, nbytes);
    }