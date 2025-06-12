fn decode_hex_escape(&mut self) -> Result<u16> {
        match self.slice[self.index..] {
            [a, b, c, d, ..] => {
                self.index += 4;
                match decode_four_hex_digits(a, b, c, d) {
                    Some(val) => Ok(val),
                    None => error(self, ErrorCode::InvalidEscape),
                }
            }
            _ => {
                self.index = self.slice.len();
                error(self, ErrorCode::EofWhileParsingString)
            }
        }
    }