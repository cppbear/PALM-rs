fn get_u128_ne(&mut self) -> u128 {
        buf_get_impl!(self, u128::from_ne_bytes);
    }