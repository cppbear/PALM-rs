fn get_u128_le(&mut self) -> u128 {
        buf_get_impl!(self, u128::from_le_bytes);
    }