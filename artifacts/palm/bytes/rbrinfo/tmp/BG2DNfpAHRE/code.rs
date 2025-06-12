fn get_u64_ne(&mut self) -> u64 {
        buf_get_impl!(self, u64::from_ne_bytes);
    }