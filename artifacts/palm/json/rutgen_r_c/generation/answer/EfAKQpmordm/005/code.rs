// Answer 0

#[test]
fn test_ignore_str_success() {
    struct MockRead {
        bytes: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, position: 0 }
        }

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                Ok(Some(self.bytes[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
    }

    impl Read<'_> for MockRead {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            self.next()
        }
        fn peek(&mut self) -> Result<Option<u8>> {
            self.peek()
        }
        fn discard(&mut self) {}
        fn position(&self) -> Position {
            Position {}
        }
        fn peek_position(&self) -> Position {
            Position {}
        }
        fn byte_offset(&self) -> usize {
            self.position
        }
        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'_, 's, str>> {
            unreachable!()
        }
        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'_, 's, [u8]>> {
            unreachable!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            loop {
                let ch = self.next()?.ok_or(Error::EofWhileParsingString)?;
                if !is_escape(ch, true) {
                    continue;
                }
                match ch {
                    b'"' => {
                        return Ok(());
                    }
                    b'\\' => {
                        // Imagine ignore_escape does something here
                    }
                    _ => {
                        return Err(Error::new(ErrorCode::ControlCharacterWhileParsingString));
                    }
                }
            }
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unreachable!()
        }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut mock_read = MockRead::new(vec![b'\\', b'"']);
    let result = mock_read.ignore_str();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_ignore_str_control_character() {
    struct MockRead {
        bytes: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, position: 0 }
        }

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                Ok(Some(self.bytes[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
    }

    impl Read<'_> for MockRead {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            self.next()
        }
        fn peek(&mut self) -> Result<Option<u8>> {
            self.peek()
        }
        fn discard(&mut self) {}
        fn position(&self) -> Position {
            Position {}
        }
        fn peek_position(&self) -> Position {
            Position {}
        }
        fn byte_offset(&self) -> usize {
            self.position
        }
        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'_, 's, str>> {
            unreachable!()
        }
        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'_, 's, [u8]>> {
            unreachable!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            loop {
                let ch = self.next()?.ok_or(Error::EofWhileParsingString)?;
                if !is_escape(ch, true) {
                    continue;
                }
                match ch {
                    b'"' => {
                        return Ok(());
                    }
                    b'\\' => {
                        // Imagine ignore_escape does something here
                    }
                    _ => {
                        panic!(); // Triggering panic intentionally
                    }
                }
            }
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unreachable!()
        }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut mock_read = MockRead::new(vec![b'\\', b'\x1b']); // Control character
    let _result = mock_read.ignore_str();
}

