fn get_i128_le(&mut self) -> i128 {
        buf_get_impl!(self, i128::from_le_bytes);
    }