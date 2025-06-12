fn is_eof(&self) -> bool {
        self.offset() == self.pattern().len()
    }