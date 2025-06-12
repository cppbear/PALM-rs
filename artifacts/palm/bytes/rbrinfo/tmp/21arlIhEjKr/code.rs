fn put_u32(&mut self, n: u32) {
        self.put_slice(&n.to_be_bytes())
    }