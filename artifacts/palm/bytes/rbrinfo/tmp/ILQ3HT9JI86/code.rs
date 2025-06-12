fn get_i128_ne(&mut self) -> i128 {
        buf_get_impl!(self, i128::from_ne_bytes);
    }