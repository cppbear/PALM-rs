fn put_i16_ne(&mut self, n: i16) {
        self.put_slice(&n.to_ne_bytes())
    }