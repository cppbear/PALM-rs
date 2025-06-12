fn put_i32_ne(&mut self, n: i32) {
        self.put_slice(&n.to_ne_bytes())
    }