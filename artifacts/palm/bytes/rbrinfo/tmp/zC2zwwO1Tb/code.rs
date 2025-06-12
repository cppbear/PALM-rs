fn get_u16_ne(&mut self) -> u16 {
        buf_get_impl!(self, u16::from_ne_bytes);
    }