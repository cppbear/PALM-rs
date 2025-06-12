fn put_u16_ne(&mut self, n: u16) {
        self.put_slice(&n.to_ne_bytes())
    }