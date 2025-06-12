fn get_u64_le(&mut self) -> u64 {
        buf_get_impl!(self, u64::from_le_bytes);
    }