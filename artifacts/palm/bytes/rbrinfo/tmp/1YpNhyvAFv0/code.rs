fn put_i64(&mut self, n: i64) {
        self.put_slice(&n.to_be_bytes())
    }