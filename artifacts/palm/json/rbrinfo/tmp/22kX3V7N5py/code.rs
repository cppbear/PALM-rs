fn parse_any_signed_number(&mut self) -> Result<ParserNumber> {
        let peek = match tri!(self.peek()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };

        let value = match peek {
            b'-' => {
                self.eat_char();
                self.parse_any_number(false)
            }
            b'0'..=b'9' => self.parse_any_number(true),
            _ => Err(self.peek_error(ErrorCode::InvalidNumber)),
        };

        let value = match tri!(self.peek()) {
            Some(_) => Err(self.peek_error(ErrorCode::InvalidNumber)),
            None => value,
        };

        match value {
            Ok(value) => Ok(value),
            // The de::Error impl creates errors with unknown line and column.
            // Fill in the position here by looking at the current index in the
            // input. There is no way to tell whether this should call `error`
            // or `peek_error` so pick the one that seems correct more often.
            // Worst case, the position is off by one character.
            Err(err) => Err(self.fix_position(err)),
        }
    }