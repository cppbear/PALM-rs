fn try_get_int_ne(&mut self, nbytes: usize) -> Result<i64, TryGetError> {
        if cfg!(target_endian = "big") {
            self.try_get_int(nbytes)
        } else {
            self.try_get_int_le(nbytes)
        }
    }