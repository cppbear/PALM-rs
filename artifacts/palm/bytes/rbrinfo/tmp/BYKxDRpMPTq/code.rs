fn put_u64(&mut self, n: u64) {
        self.put_slice(&n.to_be_bytes())
    }