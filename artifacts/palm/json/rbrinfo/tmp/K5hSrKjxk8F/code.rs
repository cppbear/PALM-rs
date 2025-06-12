fn next_char(&mut self) -> Result<Option<u8>> {
        self.read.next()
    }