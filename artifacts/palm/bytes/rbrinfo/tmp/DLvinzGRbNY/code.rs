fn consume(&mut self, amt: usize) {
        self.buf.advance(amt)
    }