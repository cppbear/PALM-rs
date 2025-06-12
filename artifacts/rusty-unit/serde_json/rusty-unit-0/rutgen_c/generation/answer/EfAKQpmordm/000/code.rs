// Answer 0

#[test]
fn test_ignore_str_success() {
    struct MockRead {
        data: Vec<u8>,
        current_index: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, current_index: 0 }
        }
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.current_index < self.data.len() {
                let byte = self.data[self.current_index];
                self.current_index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.current_index < self.data.len() {
                Ok(Some(self.data[self.current_index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.current_index = self.data.len();
        }

        fn position(&self) -> Position {
            Position { line: 1, col: self.current_index }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, col: self.current_index }
        }

        fn byte_offset(&self) -> usize {
            self.current_index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            loop {
                let ch = self.next()?.ok_or(ErrorCode::EofWhileParsingString)?;
                if !is_escape(ch, true) {
                    continue;
                }
                match ch {
                    b'"' => {
                        return Ok(());
                    }
                    b'\\' => {
                        // Simulate ignoring escape
                        continue;
                    }
                    _ => {
                        return Err(Error { err: Box::new(ErrorCode::ControlCharacterWhileParsingString.into()) });
                    }
                }
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MockRead::new(vec![b'\\', b'"']);
    assert!(reader.ignore_str().is_ok());
}

#[test]
#[should_panic]
fn test_ignore_str_control_character_error() {
    struct MockRead {
        data: Vec<u8>,
        current_index: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, current_index: 0 }
        }
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.current_index < self.data.len() {
                let byte = self.data[self.current_index];
                self.current_index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.current_index < self.data.len() {
                Ok(Some(self.data[self.current_index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.current_index = self.data.len();
        }

        fn position(&self) -> Position {
            Position { line: 1, col: self.current_index }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, col: self.current_index }
        }

        fn byte_offset(&self) -> usize {
            self.current_index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            loop {
                let ch = self.next()?.ok_or(ErrorCode::EofWhileParsingString)?;
                if !is_escape(ch, true) {
                    continue;
                }
                match ch {
                    b'"' => {
                        return Ok(());
                    }
                    b'\\' => {
                        continue;
                    }
                    _ => {
                        return Err(Error { err: Box::new(ErrorCode::ControlCharacterWhileParsingString.into()) });
                    }
                }
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MockRead::new(vec![b'\\', b'\0']);
    let _ = reader.ignore_str(); // Should panic on control character error
}

