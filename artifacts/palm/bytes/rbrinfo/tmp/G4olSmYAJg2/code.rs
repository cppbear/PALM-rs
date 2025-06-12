fn put_u64_le(&mut self, n: u64) {
        self.put_slice(&n.to_le_bytes())
    }