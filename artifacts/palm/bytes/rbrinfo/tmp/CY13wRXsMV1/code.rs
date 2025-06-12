fn get_int_ne(&mut self, nbytes: usize) -> i64 {
        if cfg!(target_endian = "big") {
            self.get_int(nbytes)
        } else {
            self.get_int_le(nbytes)
        }
    }