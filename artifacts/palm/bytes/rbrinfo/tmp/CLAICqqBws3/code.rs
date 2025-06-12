fn get_i32(&mut self) -> i32 {
        buf_get_impl!(self, i32::from_be_bytes);
    }