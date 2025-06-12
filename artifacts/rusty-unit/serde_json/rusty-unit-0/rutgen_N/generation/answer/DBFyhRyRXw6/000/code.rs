// Answer 0

#[test]
fn test_parse_exponent_positive_significand() {
    struct TestParser {
        input: &'static [u8],
        position: usize,
    }

    impl TestParser {
        fn new(input: &'static [u8]) -> Self {
            TestParser { input, position: 0 }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn peek_or_null(&self) -> Option<u8> {
            self.input.get(self.position).copied()
        }

        fn next_char(&mut self) -> Option<u8> {
            self.peek_or_null().map(|c| {
                self.eat_char();
                c
            })
        }

        fn error(&self, _code: ErrorCode) -> Error {
            // Implementation of error handling
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64> {
            // Implementation that converts parts to f64
        }
    }

    let mut parser = TestParser::new(b"E+1");
    let result = parser.parse_exponent(true, 123, 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1.23e2); // Expected result based on the implementation
}

#[test]
fn test_parse_exponent_negative_exp() {
    struct TestParser {
        input: &'static [u8],
        position: usize,
    }

    impl TestParser {
        fn new(input: &'static [u8]) -> Self {
            TestParser { input, position: 0 }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn peek_or_null(&self) -> Option<u8> {
            self.input.get(self.position).copied()
        }

        fn next_char(&mut self) -> Option<u8> {
            self.peek_or_null().map(|c| {
                self.eat_char();
                c
            })
        }

        fn error(&self, _code: ErrorCode) -> Error {
            // Implementation of error handling
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64> {
            // Implementation that converts parts to f64
        }
    }

    let mut parser = TestParser::new(b"E-1");
    let result = parser.parse_exponent(true, 123, 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1.23e1); // Expected result based on the implementation
}

#[test]
#[should_panic]
fn test_parse_exponent_invalid_character() {
    struct TestParser {
        input: &'static [u8],
        position: usize,
    }

    impl TestParser {
        fn new(input: &'static [u8]) -> Self {
            TestParser { input, position: 0 }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn peek_or_null(&self) -> Option<u8> {
            self.input.get(self.position).copied()
        }

        fn next_char(&mut self) -> Option<u8> {
            self.peek_or_null().map(|c| {
                self.eat_char();
                c
            })
        }

        fn error(&self, _code: ErrorCode) -> Error {
            // Implementation of error handling
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64> {
            // Implementation that converts parts to f64
        }
    }

    let mut parser = TestParser::new(b"E+X");
    let _ = parser.parse_exponent(true, 123, 2);
}

#[test]
fn test_parse_exponent_overflow() {
    struct TestParser {
        input: &'static [u8],
        position: usize,
    }

    impl TestParser {
        fn new(input: &'static [u8]) -> Self {
            TestParser { input, position: 0 }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn peek_or_null(&self) -> Option<u8> {
            self.input.get(self.position).copied()
        }

        fn next_char(&mut self) -> Option<u8> {
            self.peek_or_null().map(|c| {
                self.eat_char();
                c
            })
        }

        fn error(&self, _code: ErrorCode) -> Error {
            // Implementation of error handling
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64> {
            // Implementation that converts parts to f64
        }

        fn parse_exponent_overflow(&self, _positive: bool, _zero_significand: bool, _positive_exp: bool) -> Result<f64> {
            // Simulate overflow handling logic
            Err(self.error(ErrorCode::Overflow))
        }
    }

    let mut parser = TestParser::new(b"E+10000000000");
    let result = parser.parse_exponent(true, 0, i32::MAX);
    assert!(result.is_err());
}

