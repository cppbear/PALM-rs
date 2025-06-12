fn peek(&mut self) -> Result<Option<u8>> {
        self.delegate.peek()
    }