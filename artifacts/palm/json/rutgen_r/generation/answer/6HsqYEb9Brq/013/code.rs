// Answer 0

#[test]
fn test_ignore_integer_with_valid_case() {
    struct MockDeserializer {
        chars: Vec<u8>,
        position: usize,
    }

    impl MockDeserializer {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.position < self.chars.len() {
                let ch = self.chars[self.position];
                self.position += 1;
                Ok(ch)
            } else {
                Err("No more chars")
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.position < self.chars.len() {
                Ok(self.chars[self.position])
            } else {
                Err("No more chars")
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.chars.len() {
                self.position += 1;
            }
        }

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            // Simulate ignoring a decimal
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            // Simulate ignoring an exponent
            Ok(())
        }

        fn error(&self, _code: ErrorCode) -> &'static str {
            "Error"
        }
    }

    impl MockDeserializer {
        fn ignore_integer(&mut self) -> Result<(), &'static str> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(self.error(ErrorCode::InvalidNumber));
                    }
                }
                b'1'..=b'9' => {
                    while let b'0'..=b'9' = self.peek_or_null()? {
                        self.eat_char();
                    }
                }
                _ => {
                    return Err(self.error(ErrorCode::InvalidNumber));
                }
            }

            match self.peek_or_null()? {
                b'.' => self.ignore_decimal(),
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }
    
    let mut deserializer = MockDeserializer {
        chars: vec![b'1', b'2', b'.'],
        position: 0,
    };

    assert_eq!(deserializer.ignore_integer(), Ok(()));
}

#[test]
fn test_ignore_integer_with_invalid_number() {
    struct MockDeserializer {
        chars: Vec<u8>,
        position: usize,
    }

    impl MockDeserializer {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.position < self.chars.len() {
                let ch = self.chars[self.position];
                self.position += 1;
                Ok(ch)
            } else {
                Err("No more chars")
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.position < self.chars.len() {
                Ok(self.chars[self.position])
            } else {
                Err("No more chars")
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.chars.len() {
                self.position += 1;
            }
        }

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            Ok(())
        }

        fn error(&self, _code: ErrorCode) -> &'static str {
            "Error"
        }
    }

    impl MockDeserializer {
        fn ignore_integer(&mut self) -> Result<(), &'static str> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(self.error(ErrorCode::InvalidNumber));
                    }
                }
                b'1'..=b'9' => {
                    while let b'0'..=b'9' = self.peek_or_null()? {
                        self.eat_char();
                    }
                }
                _ => {
                    return Err(self.error(ErrorCode::InvalidNumber));
                }
            }

            match self.peek_or_null()? {
                b'.' => self.ignore_decimal(),
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut deserializer = MockDeserializer {
        chars: vec![b'0', b'0'],
        position: 0,
    };

    assert_eq!(deserializer.ignore_integer(), Err("Error"));
}

#[test]
fn test_ignore_integer_with_leading_zero() {
    struct MockDeserializer {
        chars: Vec<u8>,
        position: usize,
    }

    impl MockDeserializer {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.position < self.chars.len() {
                let ch = self.chars[self.position];
                self.position += 1;
                Ok(ch)
            } else {
                Err("No more chars")
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.position < self.chars.len() {
                Ok(self.chars[self.position])
            } else {
                Err("No more chars")
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.chars.len() {
                self.position += 1;
            }
        }

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            Ok(())
        }

        fn error(&self, _code: ErrorCode) -> &'static str {
            "Error"
        }
    }

    impl MockDeserializer {
        fn ignore_integer(&mut self) -> Result<(), &'static str> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(self.error(ErrorCode::InvalidNumber));
                    }
                }
                b'1'..=b'9' => {
                    while let b'0'..=b'9' = self.peek_or_null()? {
                        self.eat_char();
                    }
                }
                _ => {
                    return Err(self.error(ErrorCode::InvalidNumber));
                }
            }

            match self.peek_or_null()? {
                b'.' => self.ignore_decimal(),
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut deserializer = MockDeserializer {
        chars: vec![b'0', b'1'],
        position: 0,
    };

    assert_eq!(deserializer.ignore_integer(), Err("Error"));
}

