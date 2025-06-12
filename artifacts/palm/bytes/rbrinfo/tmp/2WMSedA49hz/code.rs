fn get_i128(&mut self) -> i128 {
        buf_get_impl!(self, i128::from_be_bytes);
    }