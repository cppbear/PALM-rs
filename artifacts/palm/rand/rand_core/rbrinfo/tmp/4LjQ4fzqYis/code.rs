fn fill_bytes(&mut self, dst: &mut [u8]) {
        self.0.try_fill_bytes(dst).unwrap()
    }