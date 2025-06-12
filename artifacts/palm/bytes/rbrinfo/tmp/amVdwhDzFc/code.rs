fn put_i32_le(&mut self, n: i32) {
        self.put_slice(&n.to_le_bytes())
    }