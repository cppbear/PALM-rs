fn get_i32_ne(&mut self) -> i32 {
        buf_get_impl!(self, i32::from_ne_bytes);
    }