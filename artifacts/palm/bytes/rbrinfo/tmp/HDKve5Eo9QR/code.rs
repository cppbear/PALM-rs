fn get_i16_ne(&mut self) -> i16 {
        buf_get_impl!(self, i16::from_ne_bytes);
    }