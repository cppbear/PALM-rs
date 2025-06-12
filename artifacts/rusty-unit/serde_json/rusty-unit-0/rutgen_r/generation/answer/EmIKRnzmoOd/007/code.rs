// Answer 0

#[test]
fn test_scan_integer128_leading_zero_invalid() {
    struct TestReader {
        input: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Error handling implementation here
        }
    }

    let mut reader = TestReader {
        input: vec![b'0', b'1'], // Leading zero followed by a digit
        index: 0,
    };
    let mut buf = String::new();
    let result = reader.scan_integer128(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_scan_integer128_multiple_leading_zeros_invalid() {
    struct TestReader {
        input: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Error handling implementation here
        }
    }

    let mut reader = TestReader {
        input: vec![b'0', b'0'], // Leading zero followed by another zero
        index: 0,
    };
    let mut buf = String::new();
    let result = reader.scan_integer128(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_scan_integer128_invalid_character() {
    struct TestReader {
        input: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Error handling implementation here
        }
    }

    let mut reader = TestReader {
        input: vec![b'1', b'a'], // Valid number followed by an invalid character
        index: 0,
    };
    let mut buf = String::new();
    let result = reader.scan_integer128(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_scan_integer128_valid_case() {
    struct TestReader {
        input: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0) // Simulating null character
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Error handling implementation here
        }
    }

    let mut reader = TestReader {
        input: vec![b'1', b'2', b'3'], // Valid input
        index: 0,
    };
    let mut buf = String::new();
    let result = reader.scan_integer128(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "123");
}

