fn put_uint_ne(&mut self, n: u64, nbytes: usize) {
        if cfg!(target_endian = "big") {
            self.put_uint(n, nbytes)
        } else {
            self.put_uint_le(n, nbytes)
        }
    }