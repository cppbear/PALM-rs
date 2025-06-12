fn parse_whitespace(&mut self) -> Result<Option<u8>> {
        loop {
            match tri!(self.peek()) {
                Some(b' ' | b'\n' | b'\t' | b'\r') => {
                    self.eat_char();
                }
                other => {
                    return Ok(other);
                }
            }
        }
    }