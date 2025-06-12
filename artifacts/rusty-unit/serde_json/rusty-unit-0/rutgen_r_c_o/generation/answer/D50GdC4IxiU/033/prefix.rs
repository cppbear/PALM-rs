// Answer 0

#[test]
fn test_parse_unicode_escape_valid_n_d800() {
    struct TestRead {
        state: usize,
    }

    impl TestRead {
        fn new() -> Self {
            Self { state: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.state == 0 {
                self.state += 1;
                Ok(0xD800)
            } else {
                Err(Error::from(ErrorCode::UnexpectedEndOfHexEscape))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Err(Error::from(ErrorCode::UnexpectedEndOfHexEscape))
        }

        fn discard(&mut self) {}
    }

    let mut read = TestRead::new();
    let validate = false;
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_valid_n_dbff() {
    struct TestRead {
        state: usize,
    }

    impl TestRead {
        fn new() -> Self {
            Self { state: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.state == 0 {
                self.state += 1;
                Ok(0xDBFF)
            } else {
                Err(Error::from(ErrorCode::UnexpectedEndOfHexEscape))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Err(Error::from(ErrorCode::UnexpectedEndOfHexEscape))
        }

        fn discard(&mut self) {}
    }

    let mut read = TestRead::new();
    let validate = false;
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut read, validate, &mut scratch);
}

