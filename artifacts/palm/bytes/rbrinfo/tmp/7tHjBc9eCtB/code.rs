fn get_uint(&mut self, nbytes: usize) -> u64 {
        buf_get_impl!(be => self, u64, nbytes);
    }