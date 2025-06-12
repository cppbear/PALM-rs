fn put_i16_le(&mut self, n: i16) {
        self.put_slice(&n.to_le_bytes())
    }