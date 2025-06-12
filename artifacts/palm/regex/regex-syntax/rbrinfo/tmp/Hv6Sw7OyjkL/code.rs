fn peek(&self) -> Option<char> {
        if self.is_eof() {
            return None;
        }
        self.pattern()[self.offset() + self.char().len_utf8()..].chars().next()
    }