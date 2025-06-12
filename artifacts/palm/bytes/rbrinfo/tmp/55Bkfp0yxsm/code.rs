fn put_u64_ne(&mut self, n: u64) {
        self.put_slice(&n.to_ne_bytes())
    }