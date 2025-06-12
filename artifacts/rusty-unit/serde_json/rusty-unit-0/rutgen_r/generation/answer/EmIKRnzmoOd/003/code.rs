// Answer 0

#[test]
fn test_scan_integer128_valid_single_digit() {
    struct TestScanner {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                let ch = self.input[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Ok(b'\0') // Null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(b'\0') // Null character
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Just a placeholder; in real scenarios, we'd return an error.
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            // Just a placeholder for error checking in peek.
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner { input: vec![b'5'], pos: 0 };
    let result = scanner.scan_integer128(&mut buf);

    assert!(result.is_ok());
    assert_eq!(buf, "5");
}

#[test]
fn test_scan_integer128_invalid_leading_zero() {
    struct TestScanner {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                let ch = self.input[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Ok(b'\0')
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(b'\0')
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Just a placeholder; in real scenarios, we'd return an error.
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            // Just a placeholder for error checking in peek.
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner { input: vec![b'0', b'1'], pos: 0 };
    let result = scanner.scan_integer128(&mut buf);

    assert!(result.is_err());
}

#[test]
fn test_scan_integer128_valid_multiple_digits() {
    struct TestScanner {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                let ch = self.input[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Ok(b'\0')
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(b'\0')
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Just a placeholder; in real scenarios, we'd return an error.
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            // Just a placeholder for error checking in peek.
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner { input: vec![b'1', b'2', b'3'], pos: 0 };
    let result = scanner.scan_integer128(&mut buf);

    assert!(result.is_ok());
    assert_eq!(buf, "123");
}

#[test]
fn test_scan_integer128_invalid_non_digit_after_zero() {
    struct TestScanner {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                let ch = self.input[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Ok(b'\0')
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(b'\0')
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Just a placeholder; in real scenarios, we'd return an error.
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            // Just a placeholder for error checking in peek.
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner { input: vec![b'0', b'a'], pos: 0 };
    let result = scanner.scan_integer128(&mut buf);

    assert!(result.is_err());
}

