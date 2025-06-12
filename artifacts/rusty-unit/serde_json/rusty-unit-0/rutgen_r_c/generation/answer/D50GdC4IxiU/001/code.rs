// Answer 0

#[test]
fn test_parse_unicode_escape_invalid_hex_escape() {
    struct MockRead {
        called_decode: bool,
        decode_result: Result<i16>,
    }

    impl MockRead {
        fn new() -> Self {
            Self {
                called_decode: false,
                decode_result: Err(Error::new(ErrorCode::InvalidEscape)),
            }
        }
    }

    impl Read<'_> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<i16> {
            self.called_decode = true;
            self.decode_result.clone()
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'u')) // Simulating the presence of next token
        }

        fn discard(&mut self) {}
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new();
    let result = parse_unicode_escape(&mut read, true, &mut scratch);
  
    assert!(result.is_err());
    assert_eq!(read.called_decode, true);
}

#[test]
fn test_parse_unicode_escape_lone_leading_surrogate() {
    struct MockRead {
        called_decode: bool,
        decode_result: Result<i16>,
    }

    impl MockRead {
        fn new() -> Self {
            Self {
                called_decode: false,
                decode_result: Ok(0xDC00),
            }
        }
    }

    impl Read<'_> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<i16> {
            self.called_decode = true;
            self.decode_result.clone()
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'u')) // Simulating the presence of next token
        }

        fn discard(&mut self) {}
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new();
    let result = parse_unicode_escape(&mut read, true, &mut scratch);

    assert!(result.is_err());
    assert_eq!(read.called_decode, true);
}

#[test]
fn test_parse_unicode_escape_leading_surrogate_followed_by_invalid() {
    struct MockRead {
        call_count: usize,
        decode_results: Vec<Result<i16>>,
    }

    impl MockRead {
        fn new() -> Self {
            Self {
                call_count: 0,
                decode_results: vec![Ok(0xD800), Err(Error::new(ErrorCode::InvalidEscape))],
            }
        }
    }

    impl Read<'_> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<i16> {
            let result = self.decode_results[self.call_count].clone();
            self.call_count += 1;
            result
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'u')) // Simulating presence of next token
        }

        fn discard(&mut self) {}
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new();
    let result = parse_unicode_escape(&mut read, true, &mut scratch);

    assert!(result.is_err());
    assert_eq!(read.call_count, 2);
}

#[test]
fn test_parse_unicode_escape_leading_surrogate_followed_by_valid() {
    struct MockRead {
        call_count: usize,
        decode_results: Vec<Result<i16>>,
    }

    impl MockRead {
        fn new() -> Self {
            Self {
                call_count: 0,
                decode_results: vec![Ok(0xD800), Ok(0xDC00)],
            }
        }
    }

    impl Read<'_> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<i16> {
            let result = self.decode_results[self.call_count].clone();
            self.call_count += 1;
            result
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'u')) // Simulating presence of next token
        }

        fn discard(&mut self) {}
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new();
    let result = parse_unicode_escape(&mut read, true, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(read.call_count, 2);
}

