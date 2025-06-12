fn get_u16(&mut self) -> u16 {
        buf_get_impl!(self, u16::from_be_bytes);
    }