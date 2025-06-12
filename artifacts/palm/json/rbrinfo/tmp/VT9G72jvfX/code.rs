fn next(&mut self) -> Result<Option<u8>> {
        self.delegate.next()
    }