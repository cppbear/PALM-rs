fn put_i16(&mut self, n: i16) {
        self.put_slice(&n.to_be_bytes())
    }