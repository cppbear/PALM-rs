fn ignore_str(&mut self) -> Result<()> {
        loop {
            self.skip_to_escape(true);
            if self.index == self.slice.len() {
                return error(self, ErrorCode::EofWhileParsingString);
            }
            match self.slice[self.index] {
                b'"' => {
                    self.index += 1;
                    return Ok(());
                }
                b'\\' => {
                    self.index += 1;
                    tri!(ignore_escape(self));
                }
                _ => {
                    return error(self, ErrorCode::ControlCharacterWhileParsingString);
                }
            }
        }
    }