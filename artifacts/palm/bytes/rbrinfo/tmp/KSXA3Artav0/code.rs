fn get_u64(&mut self) -> u64 {
        buf_get_impl!(self, u64::from_be_bytes);
    }