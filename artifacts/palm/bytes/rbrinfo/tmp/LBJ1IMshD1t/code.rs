fn get_i64_le(&mut self) -> i64 {
        buf_get_impl!(self, i64::from_le_bytes);
    }