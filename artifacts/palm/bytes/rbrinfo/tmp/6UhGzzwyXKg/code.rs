fn get_i32_le(&mut self) -> i32 {
        buf_get_impl!(self, i32::from_le_bytes);
    }