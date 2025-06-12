fn put_i8(&mut self, n: i8) {
        let src = [n as u8];
        self.put_slice(&src)
    }