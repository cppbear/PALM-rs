// Answer 0

#[test]
fn test_scan_integer128_single_zero() {
    struct TestScanner {
        chars: Vec<u8>,
        pos: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                self.pos += 1;
                Ok(c)
            } else {
                Ok(0) // Null character
            }
        }
        
        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                Ok(self.chars[self.pos])
            } else {
                Ok(0) // Null character
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1; // Simulate consuming a character
        }

        fn error(&self, _: ErrorCode) -> &'static str {
            "Error"
        }

        fn peek_error(&self, _: ErrorCode) -> &'static str {
            "Peek Error"
        }
    }
    
    let mut scanner = TestScanner { chars: vec![b'0'], pos: 0 };
    let mut buf = String::new();
    let result = scanner.scan_integer128(&mut buf);
    assert_eq!(result, Err("Peek Error"));
}

#[test]
fn test_scan_integer128_invalid_leading_zero() {
    struct TestScanner {
        chars: Vec<u8>,
        pos: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                self.pos += 1;
                Ok(c)
            } else {
                Ok(0) // Null character
            }
        }
        
        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                Ok(self.chars[self.pos])
            } else {
                Ok(0) // Null character
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1; // Simulate consuming a character
        }

        fn error(&self, _: ErrorCode) -> &'static str {
            "Error"
        }

        fn peek_error(&self, _: ErrorCode) -> &'static str {
            "Peek Error"
        }
    }

    let mut scanner = TestScanner { chars: vec![b'0', b'5'], pos: 0 };
    let mut buf = String::new();
    let result = scanner.scan_integer128(&mut buf);
    assert_eq!(result, Err("Peek Error"));
}

#[test]
fn test_scan_integer128_valid_number() {
    struct TestScanner {
        chars: Vec<u8>,
        pos: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                self.pos += 1;
                Ok(c)
            } else {
                Ok(0) // Null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                Ok(self.chars[self.pos])
            } else {
                Ok(0) // Null character
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1; // Simulate consuming a character
        }

        fn error(&self, _: ErrorCode) -> &'static str {
            "Error"
        }

        fn peek_error(&self, _: ErrorCode) -> &'static str {
            "Peek Error"
        }
    }

    let mut scanner = TestScanner { chars: vec![b'1', b'2', b'3'], pos: 0 };
    let mut buf = String::new();
    let result = scanner.scan_integer128(&mut buf);
    assert_eq!(result, Ok(()));
    assert_eq!(buf, "123");
}

