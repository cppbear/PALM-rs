fn put_i64_ne(&mut self, n: i64) {
        self.put_slice(&n.to_ne_bytes())
    }