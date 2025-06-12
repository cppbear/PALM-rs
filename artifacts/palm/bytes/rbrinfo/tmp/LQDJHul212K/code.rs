fn put_i32(&mut self, n: i32) {
        self.put_slice(&n.to_be_bytes())
    }