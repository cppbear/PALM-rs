fn next_char_or_null(&mut self) -> Result<u8> {
        Ok(tri!(self.next_char()).unwrap_or(b'\x00'))
    }