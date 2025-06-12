fn put_u16(&mut self, n: u16) {
        self.put_slice(&n.to_be_bytes())
    }