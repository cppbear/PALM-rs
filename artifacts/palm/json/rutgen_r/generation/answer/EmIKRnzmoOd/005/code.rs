// Answer 0

#[test]
fn test_scan_integer128_with_valid_integer() {
    struct MockScanner {
        input: Vec<u8>,
        position: usize,
    }

    impl MockScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(val)
            } else {
                Ok(0) // Null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0) // Null character
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn error(&self, _: ErrorCode) -> () {
            // Mock error handling, no-op
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            // Mock peek error handling, no-op
        }
    }

    let mut scanner = MockScanner {
        input: vec![b'1', b'2', b'3'], // Valid integer
        position: 0,
    };
    
    let mut buf = String::new();
    let result = scanner.scan_integer128(&mut buf);
    assert_eq!(result, Ok(()));
    assert_eq!(buf, "123");
}

#[test]
fn test_scan_integer128_with_leading_zero() {
    struct MockScanner {
        input: Vec<u8>,
        position: usize,
    }

    impl MockScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(val)
            } else {
                Ok(0) // Null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0) // Null character
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn error(&self, _: ErrorCode) -> () {
            // Mock error handling, no-op
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            // Mock peek error handling, no-op
        }
    }

    let mut scanner = MockScanner {
        input: vec![b'0', b'1'], // Leading zero followed by another digit
        position: 0,
    };
    
    let mut buf = String::new();
    let result = scanner.scan_integer128(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_scan_integer128_with_invalid_characters() {
    struct MockScanner {
        input: Vec<u8>,
        position: usize,
    }

    impl MockScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(val)
            } else {
                Ok(0) // Null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0) // Null character
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn error(&self, _: ErrorCode) -> () {
            // Mock error handling, no-op
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            // Mock peek error handling, no-op
        }
    }

    let mut scanner = MockScanner {
        input: vec![b'1', b'2', b'9', b'a'], // Valid number followed by invalid character
        position: 0,
    };
    
    let mut buf = String::new();
    let result = scanner.scan_integer128(&mut buf);
    assert!(result.is_err());
}

