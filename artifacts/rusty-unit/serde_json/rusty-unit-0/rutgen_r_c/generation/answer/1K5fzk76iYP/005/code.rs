// Answer 0

#[test]
fn test_decode_hex_escape_valid() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingValue))
            }
        }
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            match self.next_or_eof() {
                Ok(byte) => Ok(Some(byte)),
                Err(_) => Ok(None),
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> { Ok(Some(self.data[self.position])) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            let a = self.next_or_eof()?;
            let b = self.next_or_eof()?;
            let c = self.next_or_eof()?;
            let d = self.next_or_eof()?;
            match decode_four_hex_digits(a, b, c, d) {
                Some(val) => Ok(val),
                None => Err(Error::from(ErrorCode::InvalidEscape)),
            }
        }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = MockReader::new(vec![0x30, 0x31, 0x32, 0x33]); // '0123' in hex
    assert_eq!(reader.decode_hex_escape(), Ok(0x123)); // Check for valid hex escape
}

