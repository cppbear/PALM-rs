// Answer 0

#[test]
fn test_scan_integer128_leading_zero() {
    struct Scanner {
        chars: Vec<u8>,
        position: usize,
    }

    impl Scanner {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.position < self.chars.len() {
                let ch = self.chars[self.position];
                self.position += 1;
                Ok(ch)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.position < self.chars.len() {
                Ok(self.chars[self.position])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn error(&self, _err: ErrorCode) -> Result<()> {
            Err("Error occurred".into())
        }
    }

    let mut buf = String::new();
    let mut scanner = Scanner {
        chars: vec![b'0'],
        position: 0,
    };

    let result = scanner.scan_integer128(&mut buf);
    assert!(result.is_err());
    assert_eq!(buf, "0");
}

#[test]
fn test_scan_integer128_invalid_leading_digit() {
    struct Scanner {
        chars: Vec<u8>,
        position: usize,
    }

    impl Scanner {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.position < self.chars.len() {
                let ch = self.chars[self.position];
                self.position += 1;
                Ok(ch)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.position < self.chars.len() {
                Ok(self.chars[self.position])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn error(&self, _err: ErrorCode) -> Result<()> {
            Err("Error occurred".into())
        }
    }

    let mut buf = String::new();
    let mut scanner = Scanner {
        chars: vec![b'0', b'1'],
        position: 0,
    };

    let result = scanner.scan_integer128(&mut buf);
    assert!(result.is_err());
    assert_eq!(buf, "0");
}

#[test]
fn test_scan_integer128_valid_number() {
    struct Scanner {
        chars: Vec<u8>,
        position: usize,
    }

    impl Scanner {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.position < self.chars.len() {
                let ch = self.chars[self.position];
                self.position += 1;
                Ok(ch)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.position < self.chars.len() {
                Ok(self.chars[self.position])
            } else {
                Ok(0)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn error(&self, _err: ErrorCode) -> Result<()> {
            Err("Error occurred".into())
        }
    }

    let mut buf = String::new();
    let mut scanner = Scanner {
        chars: vec![b'1', b'2', b'3'],
        position: 0,
    };

    let result = scanner.scan_integer128(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "123");
}

