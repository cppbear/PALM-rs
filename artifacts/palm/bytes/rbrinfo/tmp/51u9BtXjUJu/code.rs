fn try_get_uint_ne(&mut self, nbytes: usize) -> Result<u64, TryGetError> {
        if cfg!(target_endian = "big") {
            self.try_get_uint(nbytes)
        } else {
            self.try_get_uint_le(nbytes)
        }
    }