// Answer 0

#[test]
fn test_ignore_str_with_valid_escape() {
    struct MockReader {
        position: usize,
        data: Vec<u8>,
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Ok(Reference::new("", 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Ok(Reference::new(&[], 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            let ch = self.next()?;
            if ch.unwrap() == b'\\' {
                return Ok(());
            }
            Err(Error::new(ErrorCode::ControlCharacterWhileParsingString))
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MockReader { position: 0, data: vec![b'\\', b'"'] };
    let _ = reader.ignore_str();
}

#[test]
fn test_ignore_str_with_control_character() {
    struct MockReader {
        position: usize,
        data: Vec<u8>,
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Ok(Reference::new("", 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Ok(Reference::new(&[], 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            let ch = self.next()?;
            if ch.is_some() && !is_escape(ch.unwrap(), true) {
                return Err(Error::new(ErrorCode::ControlCharacterWhileParsingString));
            }
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MockReader { position: 0, data: vec![b'\x01'] };
    let result = reader.ignore_str();
    assert!(result.is_err());
}

