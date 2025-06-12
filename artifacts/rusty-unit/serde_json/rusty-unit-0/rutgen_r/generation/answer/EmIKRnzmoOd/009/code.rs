// Answer 0

#[test]
fn test_scan_integer128_leading_zero() {
    struct TestScanner {
        input: Vec<u8>,
        cursor: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.cursor < self.input.len() {
                let c = self.input[self.cursor];
                self.cursor += 1;
                Ok(c)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.cursor < self.input.len() {
                Ok(self.input[self.cursor])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.cursor += 1;
        }

        fn error(&self, _: ErrorCode) -> () {
            // Assuming some error handling
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            // Assuming some error handling
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner {
        input: vec![b'0', b'1'], // leading zero followed by another digit
        cursor: 0,
    };

    let result = scanner.scan_integer128(&mut buf);
    assert_eq!(result, Err(scanner.peek_error(ErrorCode::InvalidNumber)));
}

#[test]
fn test_scan_integer128_valid_number() {
    struct TestScanner {
        input: Vec<u8>,
        cursor: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.cursor < self.input.len() {
                let c = self.input[self.cursor];
                self.cursor += 1;
                Ok(c)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.cursor < self.input.len() {
                Ok(self.input[self.cursor])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.cursor += 1;
        }

        fn error(&self, _: ErrorCode) -> () {
            // Assuming some error handling
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            // Assuming some error handling
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner {
        input: vec![b'1', b'2', b'3'], // valid number
        cursor: 0,
    };

    let result = scanner.scan_integer128(&mut buf);
    assert_eq!(result, Ok(()));
    assert_eq!(buf, "123");
}

#[test]
fn test_scan_integer128_invalid_digit_after_zero() {
    struct TestScanner {
        input: Vec<u8>,
        cursor: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.cursor < self.input.len() {
                let c = self.input[self.cursor];
                self.cursor += 1;
                Ok(c)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.cursor < self.input.len() {
                Ok(self.input[self.cursor])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.cursor += 1;
        }

        fn error(&self, _: ErrorCode) -> () {
            // Assuming some error handling
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            // Assuming some error handling
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner {
        input: vec![b'0', b'5'], // leading zero followed by invalid digit
        cursor: 0,
    };

    let result = scanner.scan_integer128(&mut buf);
    assert_eq!(result, Err(scanner.peek_error(ErrorCode::InvalidNumber)));
}

