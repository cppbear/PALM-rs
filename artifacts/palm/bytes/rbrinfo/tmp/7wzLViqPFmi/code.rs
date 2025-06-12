fn get_u128(&mut self) -> u128 {
        buf_get_impl!(self, u128::from_be_bytes);
    }