// Answer 0

#[test]
fn test_parse_integer_zero_leading() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, _positive: bool, value: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(value))
        }

        fn error(&self, _code: ErrorCode) -> () {
            ()
        }
    }

    let mut parser = TestParser {
        input: vec![b'0'],
        index: 0,
    };
    let result = parser.parse_integer(true);
    assert_eq!(result, Ok(ParserNumber::U64(0)));
}

#[test]
fn test_parse_integer_valid() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, _positive: bool, value: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(value))
        }

        fn error(&self, _code: ErrorCode) -> () {
            ()
        }
    }

    let mut parser = TestParser {
        input: vec![b'3', b'4', b'5'],
        index: 0,
    };
    let result = parser.parse_integer(true);
    assert_eq!(result, Ok(ParserNumber::U64(345)));
}

#[test]
fn test_parse_integer_invalid() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, _positive: bool, _value: u64) -> Result<ParserNumber, ()> {
            Err(())
        }

        fn error(&self, _code: ErrorCode) -> () {
            ()
        }
    }

    let mut parser = TestParser {
        input: vec![b'0', b'1'],
        index: 0,
    };
    let result = parser.parse_integer(true);
    assert!(result.is_err());
}

#[test]
fn test_parse_integer_zero_followed_by_digit() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn parse_number(&self, _positive: bool, _value: u64) -> Result<ParserNumber, ()> {
            Err(())
        }

        fn error(&self, _code: ErrorCode) -> () {
            ()
        }
    }

    let mut parser = TestParser {
        input: vec![b'0', b'1'],
        index: 0,
    };
    let result = parser.parse_integer(true);
    assert!(result.is_err());
}

