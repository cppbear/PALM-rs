fn put_u32_le(&mut self, n: u32) {
        self.put_slice(&n.to_le_bytes())
    }