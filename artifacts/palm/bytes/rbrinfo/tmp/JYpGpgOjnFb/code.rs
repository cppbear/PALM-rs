fn get_i16_le(&mut self) -> i16 {
        buf_get_impl!(self, i16::from_le_bytes);
    }