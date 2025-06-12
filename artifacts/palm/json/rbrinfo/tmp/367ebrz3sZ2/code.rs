fn end_map(&mut self) -> Result<()> {
        match tri!(self.parse_whitespace()) {
            Some(b'}') => {
                self.eat_char();
                Ok(())
            }
            Some(b',') => Err(self.peek_error(ErrorCode::TrailingComma)),
            Some(_) => Err(self.peek_error(ErrorCode::TrailingCharacters)),
            None => Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
        }
    }