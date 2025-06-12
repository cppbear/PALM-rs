fn get_u16_le(&mut self) -> u16 {
        buf_get_impl!(self, u16::from_le_bytes);
    }