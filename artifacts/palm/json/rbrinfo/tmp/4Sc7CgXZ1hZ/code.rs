pub(crate) fn peek(&mut self) -> Result<Option<u8>> {
        self.read.peek()
    }