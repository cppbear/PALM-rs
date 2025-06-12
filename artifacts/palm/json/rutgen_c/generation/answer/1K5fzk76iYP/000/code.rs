// Answer 0

#[test]
fn test_decode_hex_escape_valid() {
    struct MockReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, position: 0 }
        }

        fn next_byte(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None) // Simulating EOF
            }
        }
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            self.next_byte()
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                Ok(Some(self.bytes[self.position]))
            } else {
                Ok(None) // Simulating EOF
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            // Mock implementation
            Position::new(self.position)
        }

        fn peek_position(&self) -> Position {
            // Mock implementation
            Position::new(self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            let a = self.next_byte()?.ok_or_else(|| Error::new(ErrorCode::EofWhileParsingString))?;
            let b = self.next_byte()?.ok_or_else(|| Error::new(ErrorCode::EofWhileParsingString))?;
            let c = self.next_byte()?.ok_or_else(|| Error::new(ErrorCode::EofWhileParsingString))?;
            let d = self.next_byte()?.ok_or_else(|| Error::new(ErrorCode::EofWhileParsingString))?;
            match decode_four_hex_digits(a, b, c, d) {
                Some(val) => Ok(val),
                None => error(self, ErrorCode::InvalidEscape),
            }
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let hex_sequence = b"dead"; // Representing 0xDEAD in hex
    let mut reader = MockReader::new(hex_sequence.to_vec());
    let result = reader.decode_hex_escape();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0xDEAD);
}

#[test]
fn test_decode_hex_escape_invalid() {
    struct MockReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, position: 0 }
        }

        fn next_byte(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None) // Simulating EOF
            }
        }
    }

    // Mock implementation of the Read trait here...

    let hex_sequence = b"deag"; // Invalid hex sequence (g is not a valid hex digit)
    let mut reader = MockReader::new(hex_sequence.to_vec());
    let result = reader.decode_hex_escape();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code(), ErrorCode::InvalidEscape);
}

#[test]
fn test_decode_hex_escape_eof() {
    struct MockReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, position: 0 }
        }

        fn next_byte(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None) // Simulating EOF
            }
        }
    }

    // Mock implementation of the Read trait here...

    let hex_sequence = b"de"; // EOF after two bytes
    let mut reader = MockReader::new(hex_sequence.to_vec());
    
    let result = reader.decode_hex_escape();
    assert!(result.is_err());
    // Ensure the error indicates EOF.
    assert_eq!(result.unwrap_err().code(), ErrorCode::EofWhileParsingString);
}

