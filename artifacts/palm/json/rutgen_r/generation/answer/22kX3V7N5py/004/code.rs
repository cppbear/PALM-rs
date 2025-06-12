// Answer 0

#[test]
fn test_parse_any_signed_number_negative() {
    struct TestParser {
        input: Vec<u8>,
        position: usize,
    }

    impl TestParser {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn parse_any_number(&self, _is_positive: bool) -> Result<ParserNumber, ()> {
            // Mock implementation for the test
            Ok(ParserNumber::from(self.input.clone()))
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            // Mock implementation
        }

        fn fix_position(&self, _err: ()) -> () {
            // Mock implementation
        }
        
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    let mut parser = TestParser::new(b"-123".to_vec());
    let result = parser.parse_any_signed_number();
    assert!(result.is_ok());
}

#[test]
fn test_parse_any_signed_number_positive() {
    struct TestParser {
        input: Vec<u8>,
        position: usize,
    }

    impl TestParser {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn parse_any_number(&self, _is_positive: bool) -> Result<ParserNumber, ()> {
            // Mock implementation for the test
            Ok(ParserNumber::from(self.input.clone()))
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            // Mock implementation
        }

        fn fix_position(&self, _err: ()) -> () {
            // Mock implementation
        }

        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    let mut parser = TestParser::new(b"456".to_vec());
    let result = parser.parse_any_signed_number();
    assert!(result.is_ok());
}

#[test]
fn test_parse_any_signed_number_invalid_character() {
    struct TestParser {
        input: Vec<u8>,
        position: usize,
    }

    impl TestParser {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn parse_any_number(&self, _is_positive: bool) -> Result<ParserNumber, ()> {
            // Mock implementation for the test
            Err(())
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            // Mock implementation
        }

        fn fix_position(&self, _err: ()) -> () {
            // Mock implementation
        }

        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    let mut parser = TestParser::new(b"abc".to_vec());
    let result = parser.parse_any_signed_number();
    assert!(result.is_err());
}

#[test]
fn test_parse_any_signed_number_eof() {
    struct TestParser {
        input: Vec<u8>,
        position: usize,
    }

    impl TestParser {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn parse_any_number(&self, _is_positive: bool) -> Result<ParserNumber, ()> {
            // Mock implementation for the test
            Ok(ParserNumber::from(self.input.clone()))
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            // Mock implementation
        }

        fn fix_position(&self, _err: ()) -> () {
            // Mock implementation
        }

        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    let mut parser = TestParser::new(b"-".to_vec());
    let result = parser.parse_any_signed_number();
    assert!(result.is_err());
}

