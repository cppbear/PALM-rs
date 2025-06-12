fn get_u32(&mut self) -> u32 {
        buf_get_impl!(self, u32::from_be_bytes);
    }