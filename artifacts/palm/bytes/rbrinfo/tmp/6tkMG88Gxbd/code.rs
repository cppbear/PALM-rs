fn put_u32_ne(&mut self, n: u32) {
        self.put_slice(&n.to_ne_bytes())
    }