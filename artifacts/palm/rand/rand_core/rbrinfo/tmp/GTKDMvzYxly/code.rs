fn fill_bytes(&mut self, dst: &mut [u8]) {
        self.deref_mut().fill_bytes(dst);
    }