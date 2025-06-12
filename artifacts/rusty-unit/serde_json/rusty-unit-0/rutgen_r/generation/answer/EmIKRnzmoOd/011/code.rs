// Answer 0

#[test]
fn test_scan_integer128_valid_single_zero() {
    struct MockScanner {
        chars: Vec<u8>,
        pos: usize,
    }

    impl MockScanner {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                self.pos += 1;
                Ok(c)
            } else {
                Ok(b'\0') // mimic null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                Ok(self.chars[self.pos])
            } else {
                Ok(b'\0') // mimic null character
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;  // just move the position
        }

        fn error(&self, _: ErrorCode) -> &'static str {
            "Invalid Number"
        }

        fn peek_error(&self, _: ErrorCode) -> &'static str {
            "Peek Error"
        }
    }

    let mut scanner = MockScanner {
        chars: vec![b'0'],
        pos: 0,
    };
    let mut buf = String::new();
    let result = scanner.scan_integer128(&mut buf);

    assert_eq!(result, Ok(()));
    assert_eq!(buf, "0");
}

#[test]
#[should_panic(expected = "Invalid Number")]
fn test_scan_integer128_invalid_leading_zero() {
    struct MockScanner {
        chars: Vec<u8>,
        pos: usize,
    }

    impl MockScanner {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                self.pos += 1;
                Ok(c)
            } else {
                Ok(b'\0') // mimic null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                Ok(self.chars[self.pos])
            } else {
                Ok(b'\0') // mimic null character
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;  // just move the position
        }

        fn error(&self, _: ErrorCode) -> &'static str {
            "Invalid Number"
        }

        fn peek_error(&self, _: ErrorCode) -> &'static str {
            "Peek Error"
        }
    }

    let mut scanner = MockScanner {
        chars: vec![b'0', b'1'], // leading '0' followed by a digit
        pos: 0,
    };
    let mut buf = String::new();
    let _ = scanner.scan_integer128(&mut buf);
}

#[test]
fn test_scan_integer128_valid_multiple_digits() {
    struct MockScanner {
        chars: Vec<u8>,
        pos: usize,
    }

    impl MockScanner {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                self.pos += 1;
                Ok(c)
            } else {
                Ok(b'\0') // mimic null character
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                Ok(self.chars[self.pos])
            } else {
                Ok(b'\0') // mimic null character
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;  // just move the position
        }

        fn error(&self, _: ErrorCode) -> &'static str {
            "Invalid Number"
        }

        fn peek_error(&self, _: ErrorCode) -> &'static str {
            "Peek Error"
        }
    }

    let mut scanner = MockScanner {
        chars: vec![b'2', b'3', b'4'], // valid multiple-digit number
        pos: 0,
    };
    let mut buf = String::new();
    let result = scanner.scan_integer128(&mut buf);

    assert_eq!(result, Ok(()));
    assert_eq!(buf, "234");
}

