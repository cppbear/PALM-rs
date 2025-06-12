// Answer 0

#[test]
fn test_decode_hex_escape_success() {
    struct MockReader {
        bytes: Vec<u8>,
        pos: usize,
    }

    impl MockReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, pos: 0 }
        }

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.bytes.len() {
                let byte = self.bytes[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.bytes.len() {
                Ok(Some(self.bytes[self.pos]))
            } else {
                Ok(None)
            }
        }
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            self.next()
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            self.peek()
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            unimplemented!()
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            let a = self.next()?.ok_or(ErrorCode::EofWhileParsingValue)?;
            let b = self.next()?.ok_or(ErrorCode::EofWhileParsingValue)?;
            let c = self.next()?.ok_or(ErrorCode::EofWhileParsingValue)?;
            let d = self.next()?.ok_or(ErrorCode::EofWhileParsingValue)?;
            match decode_four_hex_digits(a, b, c, d) {
                Some(val) => Ok(val),
                None => error(self, ErrorCode::InvalidEscape),
            }
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = MockReader::new(vec![0x41, 0x42, 0x43, 0x44]); // "ABCD" in hex
    let result = reader.decode_hex_escape();
    assert_eq!(result, Ok(0xABCD));
}

#[test]
fn test_decode_hex_escape_invalid_escape() {
    struct MockReader {
        bytes: Vec<u8>,
        pos: usize,
    }

    impl MockReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, pos: 0 }
        }

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.bytes.len() {
                let byte = self.bytes[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.bytes.len() {
                Ok(Some(self.bytes[self.pos]))
            } else {
                Ok(None)
            }
        }
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            self.next()
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            self.peek()
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            unimplemented!()
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            let a = self.next()?.ok_or(ErrorCode::EofWhileParsingValue)?;
            let b = self.next()?.ok_or(ErrorCode::EofWhileParsingValue)?;
            let c = self.next()?.ok_or(ErrorCode::EofWhileParsingValue)?;
            let d = self.next()?.ok_or(ErrorCode::EofWhileParsingValue)?;
            match decode_four_hex_digits(a, b, c, d) {
                Some(val) => Ok(val),
                None => error(self, ErrorCode::InvalidEscape),
            }
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = MockReader::new(vec![0x41, 0x42, 0x43]); // Incomplete hex digits
    let result = reader.decode_hex_escape();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.code, ErrorCode::InvalidEscape);
    }
}

