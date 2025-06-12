// Answer 0

fn test_scan_integer128_leading_zero() {
    struct MockSelf {
        next_char_index: usize,
        chars: Vec<u8>,
    }

    impl MockSelf {
        fn next_char_or_null(&mut self) -> Result<u8, String> {
            if self.next_char_index < self.chars.len() {
                let ch = self.chars[self.next_char_index];
                self.next_char_index += 1;
                Ok(ch)
            } else {
                Ok(b'\0') // Simulate end of input
            }
        }

        fn peek_or_null(&self) -> Result<u8, String> {
            if self.next_char_index < self.chars.len() {
                Ok(self.chars[self.next_char_index])
            } else {
                Err("End of Stream".to_string())
            }
        }

        fn eat_char(&mut self) {
            self.next_char_index += 1;
        }

        fn error(&self, _code: ErrorCode) -> String {
            "Invalid number".to_string()
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<(), String> {
            match self.next_char_or_null()? {
                b'0' => {
                    buf.push('0');
                    match self.peek_or_null()? {
                        b'0'..=b'9' => Err(self.error(ErrorCode::InvalidNumber)),
                        _ => Ok(()),
                    }
                }
                c @ b'1'..=b'9' => {
                    buf.push(c as char);
                    while let c @ b'0'..=b'9' = self.peek_or_null()? {
                        self.eat_char();
                        buf.push(c as char);
                    }
                    Ok(())
                }
                _ => Err(self.error(ErrorCode::InvalidNumber)),
            }
        }
    }

    let mut buf = String::new();
    let mut mock_self = MockSelf {
        next_char_index: 0,
        chars: vec![b'0'],
    };
    
    let result = mock_self.scan_integer128(&mut buf);
    assert_eq!(result, Ok(()));
}

fn test_scan_integer128_invalid_leading_zero() {
    struct MockSelf {
        next_char_index: usize,
        chars: Vec<u8>,
    }

    impl MockSelf {
        fn next_char_or_null(&mut self) -> Result<u8, String> {
            if self.next_char_index < self.chars.len() {
                let ch = self.chars[self.next_char_index];
                self.next_char_index += 1;
                Ok(ch)
            } else {
                Ok(b'\0') // Simulate end of input
            }
        }

        fn peek_or_null(&self) -> Result<u8, String> {
            if self.next_char_index < self.chars.len() {
                Ok(self.chars[self.next_char_index])
            } else {
                Err("End of Stream".to_string())
            }
        }

        fn eat_char(&mut self) {
            self.next_char_index += 1;
        }

        fn error(&self, _code: ErrorCode) -> String {
            "Invalid number".to_string()
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<(), String> {
            match self.next_char_or_null()? {
                b'0' => {
                    buf.push('0');
                    match self.peek_or_null()? {
                        b'0'..=b'9' => Err(self.error(ErrorCode::InvalidNumber)),
                        _ => Ok(()),
                    }
                }
                c @ b'1'..=b'9' => {
                    buf.push(c as char);
                    while let c @ b'0'..=b'9' = self.peek_or_null()? {
                        self.eat_char();
                        buf.push(c as char);
                    }
                    Ok(())
                }
                _ => Err(self.error(ErrorCode::InvalidNumber)),
            }
        }
    }

    let mut buf = String::new();
    let mut mock_self = MockSelf {
        next_char_index: 0,
        chars: vec![b'0', b'1'], // Following 0 with an invalid number
    };
    
    let result = mock_self.scan_integer128(&mut buf);
    assert_eq!(result, Err("Invalid number".to_string()));
}

