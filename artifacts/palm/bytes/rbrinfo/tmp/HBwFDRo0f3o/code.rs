fn get_u32_ne(&mut self) -> u32 {
        buf_get_impl!(self, u32::from_ne_bytes);
    }