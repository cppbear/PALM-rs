fn put_u16_le(&mut self, n: u16) {
        self.put_slice(&n.to_le_bytes())
    }