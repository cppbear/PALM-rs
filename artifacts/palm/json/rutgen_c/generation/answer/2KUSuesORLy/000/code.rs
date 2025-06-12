// Answer 0

#[test]
fn test_decode_hex_escape_valid() {
    struct TestDelegate {
        data: Vec<u8>,
        index: usize,
    }

    impl TestDelegate {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index + 2 <= self.data.len() {
                let hex_bytes = &self.data[self.index..self.index + 2];
                self.index += 2;
                let hex_str = str::from_utf8(hex_bytes).map_err(|_| Error::new(ErrorCode::InvalidInput))?;
                u16::from_str_radix(hex_str, 16).map_err(|_| Error::new(ErrorCode::InvalidInput))
            } else {
                Err(Error::new(ErrorCode::UnexpectedEndOfInput))
            }
        }
    }

    struct TestRead<'a> {
        delegate: TestDelegate,
    }

    impl<'a> Read<'a> for TestRead<'a> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0)
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'a, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            self.delegate.decode_hex_escape()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let data = b"1a".to_vec();
    let mut delegate = TestDelegate::new(data);
    let mut reader = TestRead { delegate };

    assert_eq!(reader.decode_hex_escape().unwrap(), 26u16);
}

#[test]
#[should_panic]
fn test_decode_hex_escape_invalid() {
    struct TestDelegate {
        data: Vec<u8>,
        index: usize,
    }
    
    impl TestDelegate {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index + 2 <= self.data.len() {
                let hex_bytes = &self.data[self.index..self.index + 2];
                self.index += 2;
                let hex_str = str::from_utf8(hex_bytes).map_err(|_| Error::new(ErrorCode::InvalidInput))?;
                u16::from_str_radix(hex_str, 16).map_err(|_| Error::new(ErrorCode::InvalidInput))
            } else {
                Err(Error::new(ErrorCode::UnexpectedEndOfInput))
            }
        }
    }

    struct TestRead<'a> {
        delegate: TestDelegate,
    }

    impl<'a> Read<'a> for TestRead<'a> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0)
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'a, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            self.delegate.decode_hex_escape()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let data = b"xyz".to_vec();
    let mut delegate = TestDelegate::new(data);
    let mut reader = TestRead { delegate };

    reader.decode_hex_escape().unwrap();
}

