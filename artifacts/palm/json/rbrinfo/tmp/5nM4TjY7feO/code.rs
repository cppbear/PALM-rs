fn peek_or_null(&mut self) -> Result<u8> {
        Ok(tri!(self.peek()).unwrap_or(b'\x00'))
    }