fn put_slice(&mut self, src: &[u8]) {
        self.extend_from_slice(src);
    }