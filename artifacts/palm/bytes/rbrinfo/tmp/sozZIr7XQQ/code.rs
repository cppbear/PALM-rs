fn put_i64_le(&mut self, n: i64) {
        self.put_slice(&n.to_le_bytes())
    }