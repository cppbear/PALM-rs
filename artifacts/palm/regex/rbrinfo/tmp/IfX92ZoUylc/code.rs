fn find(&self, text: &[u8]) -> Option<usize> {
        match self.dense.len() {
            0 => None,
            1 => memchr(self.dense[0], text),
            2 => memchr2(self.dense[0], self.dense[1], text),
            3 => memchr3(self.dense[0], self.dense[1], self.dense[2], text),
            _ => self._find(text),
        }
    }