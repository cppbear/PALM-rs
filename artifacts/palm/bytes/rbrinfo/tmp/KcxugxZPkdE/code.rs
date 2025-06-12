fn get_i64(&mut self) -> i64 {
        buf_get_impl!(self, i64::from_be_bytes);
    }