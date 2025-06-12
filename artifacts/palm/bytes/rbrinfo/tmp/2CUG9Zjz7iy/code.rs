fn get_uint_le(&mut self, nbytes: usize) -> u64 {
        buf_get_impl!(le => self, u64, nbytes);
    }