// Answer 0

#[test]
fn test_decode_hex_escape_success() {
    struct TestReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(bytes: Vec<u8>) -> Self {
            TestReader { bytes, position: 0 }
        }

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingValue))
            }
        }
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            self.next_or_eof().map(Some)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                Ok(Some(self.bytes[self.position]))
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

        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

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

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader::new(vec![0x31, 0x32, 0x33, 0x34]); // '1234' in hex
    let result = reader.decode_hex_escape();
    assert_eq!(result.unwrap(), 0x1234);
}

#[test]
fn test_decode_hex_escape_failure() {
    struct TestReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(bytes: Vec<u8>) -> Self {
            TestReader { bytes, position: 0 }
        }

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingValue))
            }
        }
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            self.next_or_eof().map(Some)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                Ok(Some(self.bytes[self.position]))
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

        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

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

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader::new(vec![0x31, 0x32, 0x33]); // '123' only, missing one hex digit
    let result = reader.decode_hex_escape();
    assert!(result.is_err()); // should return an error
}

