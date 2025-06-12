fn ignore_str(&mut self) -> Result<()> {
        loop {
            let ch = tri!(next_or_eof(self));
            if !is_escape(ch, true) {
                continue;
            }
            match ch {
                b'"' => {
                    return Ok(());
                }
                b'\\' => {
                    tri!(ignore_escape(self));
                }
                _ => {
                    return error(self, ErrorCode::ControlCharacterWhileParsingString);
                }
            }
        }
    }