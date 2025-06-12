fn get_i64_ne(&mut self) -> i64 {
        buf_get_impl!(self, i64::from_ne_bytes);
    }