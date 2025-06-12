fn get_i16(&mut self) -> i16 {
        buf_get_impl!(self, i16::from_be_bytes);
    }