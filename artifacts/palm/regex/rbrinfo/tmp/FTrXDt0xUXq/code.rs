fn byte_class(&self, b: Byte) -> usize {
        match b.as_byte() {
            None => self.num_byte_classes() - 1,
            Some(b) => self.u8_class(b),
        }
    }