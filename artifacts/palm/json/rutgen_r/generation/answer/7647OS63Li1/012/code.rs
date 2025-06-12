// Answer 0

#[test]
fn test_parse_integer_zero_invalid() {
    struct Parser {
        next_char_return: Result<Option<u8>, ()>,
        peek_or_null_return: Result<u8, ()>,
        parse_long_integer_return: Result<u64, ()>,
        error_code: ErrorCode,
    }

    impl Parser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            self.next_char_return.clone()
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            self.peek_or_null_return.clone()
        }

        fn parse_long_integer(&mut self, _: bool, _: u64) -> Result<u64, ()> {
            self.parse_long_integer_return.clone()
        }
        
        fn error(&self, code: ErrorCode) -> () {
            self.error_code = code;
        }
    }

    let mut parser = Parser {
        next_char_return: Ok(Some(b'0')),
        peek_or_null_return: Ok(b'1'), // Should trigger invalid number error
        parse_long_integer_return: Ok(0),
        error_code: ErrorCode::EofWhileParsingValue,
    };

    let result = parser.parse_integer(true);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err, parser.error(ErrorCode::InvalidNumber));
    }
}

#[test]
fn test_parse_integer_out_of_bounds() {
    struct Parser {
        next_char_return: Result<Option<u8>, ()>,
        peek_or_null_return: Result<u8, ()>,
        parse_long_integer_return: Result<u64, ()>,
        error_code: ErrorCode,
    }

    impl Parser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            self.next_char_return.clone()
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            self.peek_or_null_return.clone()
        }

        fn parse_long_integer(&mut self, _: bool, _: u64) -> Result<u64, ()> {
            self.parse_long_integer_return.clone()
        }

        fn error(&self, code: ErrorCode) -> () {
            self.error_code = code;
        }
    }

    let mut parser = Parser {
        next_char_return: Ok(Some(b'1')),
        peek_or_null_return: Ok(b'0'), // Valid digit to process
        parse_long_integer_return: Ok(u64::MAX), // This will trigger the overflow
        error_code: ErrorCode::EofWhileParsingValue,
    };

    let result = parser.parse_integer(true);
    assert!(result.is_ok()); // It should return a valid result
}

#[test]
fn test_parse_integer_non_digit_after_zero() {
    struct Parser {
        next_char_return: Result<Option<u8>, ()>,
        peek_or_null_return: Result<u8, ()>,
        parse_long_integer_return: Result<u64, ()>,
        error_code: ErrorCode,
    }

    impl Parser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            self.next_char_return.clone()
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            self.peek_or_null_return.clone()
        }

        fn parse_long_integer(&mut self, _: bool, _: u64) -> Result<u64, ()> {
            self.parse_long_integer_return.clone()
        }

        fn error(&self, code: ErrorCode) -> () {
            self.error_code = code;
        }
    }

    let mut parser = Parser {
        next_char_return: Ok(Some(b'0')),
        peek_or_null_return: Ok(b'a'), // Non-digit after zero should trigger an error
        parse_long_integer_return: Ok(0),
        error_code: ErrorCode::EofWhileParsingValue,
    };

    let result = parser.parse_integer(true);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err, parser.error(ErrorCode::InvalidNumber));
    }
}

