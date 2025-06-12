// Answer 0

fn test_ignore_value_success_case() {
    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Ok(Reference::new("mock_str"))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Ok(Reference::new(b"mock_str"))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let data = vec![b't', b'r', b'u', b'e', b' ', b',', b'\n'];
    let mut mock_read = MockRead { data, index: 0 };
    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 8,
    };

    let result = deserializer.ignore_value();
    assert!(result.is_ok());
}

fn test_ignore_value_err_case() {
    struct MockReadErr {
        data: Vec<u8>,
        index: usize,
    }

    impl Read<'static> for MockReadErr {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Ok(Reference::new(b"mock_str"))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let data = vec![b'n', b'u', b'l', b'l'];
    let mut mock_read = MockReadErr { data, index: 0 };
    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 8,
    };

    let result = deserializer.ignore_value();
    assert!(result.is_err());
}

