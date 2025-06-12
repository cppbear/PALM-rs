fn advance(&mut self, cnt: usize) {
        self.drain(..cnt);
    }