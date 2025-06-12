fn get_u32_le(&mut self) -> u32 {
        buf_get_impl!(self, u32::from_le_bytes);
    }