// Answer 0

#[test]
fn test_scan_integer128_with_leading_zero() {
    struct TestScanner {
        input: Vec<u8>,
        index: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ErrorCode> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // Represents null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ErrorCode> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0) // Represents null character
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::InvalidNumber
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner {
        input: vec![b'0', b'1'], // Input to scan begins with '0'
        index: 0,
    };

    let result = scanner.scan_integer128(&mut buf);
    assert_eq!(result, Ok(()));
    assert_eq!(buf, "0");
}

#[test]
fn test_scan_integer128_with_valid_number() {
    struct TestScanner {
        input: Vec<u8>,
        index: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ErrorCode> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // Represents null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ErrorCode> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0) // Represents null character
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::InvalidNumber
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner {
        input: vec![b'1', b'2', b'3'], // Input starts with '1' followed by digits
        index: 0,
    };

    let result = scanner.scan_integer128(&mut buf);
    assert_eq!(result, Ok(()));
    assert_eq!(buf, "123");
}

#[test]
fn test_scan_integer128_with_invalid_leading_zero() {
    struct TestScanner {
        input: Vec<u8>,
        index: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ErrorCode> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // Represents null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ErrorCode> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0) // Represents null character
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::InvalidNumber
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner {
        input: vec![b'0', b'1'], // Invalid input since it leads with '0' followed by a valid digit
        index: 0,
    };

    let result = scanner.scan_integer128(&mut buf);
    assert!(result.is_err()); // Expect an error due to invalid number
}

