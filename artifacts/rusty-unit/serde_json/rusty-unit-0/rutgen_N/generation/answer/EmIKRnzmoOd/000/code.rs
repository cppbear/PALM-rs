// Answer 0

#[test]
fn test_scan_integer128_zero() {
    struct TestScanner {
        input: Vec<u8>,
        position: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let c = self.input[self.position];
                self.position += 1;
                Ok(c)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn error(&self, _code: ErrorCode) -> Result<()> {
            Err(())
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner { input: vec![b'0'], position: 0 };
    let result = scanner.scan_integer128(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "0");
}

#[test]
fn test_scan_integer128_leading_zero() {
    struct TestScanner {
        input: Vec<u8>,
        position: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let c = self.input[self.position];
                self.position += 1;
                Ok(c)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn error(&self, _code: ErrorCode) -> Result<()> {
            Err(())
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner { input: vec![b'0', b'1'], position: 0 };
    let result = scanner.scan_integer128(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_scan_integer128_valid_number() {
    struct TestScanner {
        input: Vec<u8>,
        position: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let c = self.input[self.position];
                self.position += 1;
                Ok(c)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn error(&self, _code: ErrorCode) -> Result<()> {
            Err(())
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner { input: vec![b'1', b'2', b'3'], position: 0 };
    let result = scanner.scan_integer128(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "123");
}

#[test]
fn test_scan_integer128_invalid_character() {
    struct TestScanner {
        input: Vec<u8>,
        position: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let c = self.input[self.position];
                self.position += 1;
                Ok(c)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn error(&self, _code: ErrorCode) -> Result<()> {
            Err(())
        }
    }

    let mut buf = String::new();
    let mut scanner = TestScanner { input: vec![b'1', b'2', b'3', b'8'], position: 0 };
    scanner.eat_char(); // consume '1'
    let result = scanner.scan_integer128(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "12");
}

