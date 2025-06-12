fn peek_end_of_value(&mut self) -> Result<()> {
        match tri!(self.de.peek()) {
            Some(b' ' | b'\n' | b'\t' | b'\r' | b'"' | b'[' | b']' | b'{' | b'}' | b',' | b':')
            | None => Ok(()),
            Some(_) => {
                let position = self.de.read.peek_position();
                Err(Error::syntax(
                    ErrorCode::TrailingCharacters,
                    position.line,
                    position.column,
                ))
            }
        }
    }