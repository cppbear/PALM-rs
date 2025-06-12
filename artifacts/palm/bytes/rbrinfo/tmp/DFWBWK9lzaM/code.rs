fn put_u8(&mut self, n: u8) {
        let src = [n];
        self.put_slice(&src);
    }