// Answer 0

#[test]
fn test_parse_str_success() {
    struct MockRead<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'de> Read<'de> for MockRead<'_> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
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
            Position { line: 0, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            let parsed_str = str::from_utf8(self.data).map_err(|_| Error::new(ErrorCode::InvalidUtf8))?;
            scratch.extend_from_slice(self.data);
            Ok(Reference::Borrowed(parsed_str))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut scratch = Vec::new();
    let input_data: &[u8] = b"test string";
    let mut reader = MockRead { data: input_data, position: 0 };

    let result = reader.parse_str(&mut scratch);
    assert!(result.is_ok());
    if let Ok(Reference::Borrowed(parsed)) = result {
        assert_eq!(parsed, "test string");
    }
}

#[test]
#[should_panic]
fn test_parse_str_invalid_utf8() {
    struct MockRead<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'de> Read<'de> for MockRead<'_> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
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
            Position { line: 0, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            let parsed_str = str::from_utf8(self.data).map_err(|_| Error::new(ErrorCode::InvalidUtf8))?;
            scratch.extend_from_slice(self.data);
            Ok(Reference::Borrowed(parsed_str))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut scratch = Vec::new();
    let input_data: &[u8] = &[0xFF, 0xFE, 0xFD]; // Invalid UTF-8 sequence
    let mut reader = MockRead { data: input_data, position: 0 };

    // This should panic due to invalid UTF-8
    let _result = reader.parse_str(&mut scratch);
}

