pub fn write_byte(&mut self, index: usize, byte: u8) {
        assert!(index < self.len());

        unsafe { self[index..].as_mut_ptr().write(byte) }
    }