// Answer 0

#[test]
fn test_ignore_integer_leading_zero_invalid() {
    struct TestStruct {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.index < self.chars.len() {
                let ch = self.chars[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // represents null
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Ok(0) // represents null
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.chars.len() {
                self.index += 1;
            }
        }

        fn ignore_decimal(&mut self) -> Result<()> { Ok(()) }
        fn ignore_exponent(&mut self) -> Result<()> { Ok(()) }
        
        fn ignore_integer(&mut self) -> Result<()> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(ErrorCode::InvalidNumber.into());
                    }
                }
                b'1'..=b'9' => {
                    while let b'0'..=b'9' = self.peek_or_null()? {
                        self.eat_char();
                    }
                }
                _ => {
                    return Err(ErrorCode::InvalidNumber.into());
                }
            }

            match self.peek_or_null()? {
                b'.' => self.ignore_decimal(),
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    #[derive(Debug)]
    enum ErrorCode {
        InvalidNumber,
    }

    let mut test_struct = TestStruct {
        chars: vec![b'0', b'0'], // leading zero followed by another zero
        index: 0,
    };

    let result = test_struct.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_valid() {
    struct TestStruct {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.index < self.chars.len() {
                let ch = self.chars[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // represents null
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Ok(0) // represents null
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.chars.len() {
                self.index += 1;
            }
        }

        fn ignore_decimal(&mut self) -> Result<()> { Ok(()) }
        fn ignore_exponent(&mut self) -> Result<()> { Ok(()) }
        
        fn ignore_integer(&mut self) -> Result<()> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(ErrorCode::InvalidNumber.into());
                    }
                }
                b'1'..=b'9' => {
                    while let b'0'..=b'9' = self.peek_or_null()? {
                        self.eat_char();
                    }
                }
                _ => {
                    return Err(ErrorCode::InvalidNumber.into());
                }
            }

            match self.peek_or_null()? {
                b'.' => self.ignore_decimal(),
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    #[derive(Debug)]
    enum ErrorCode {
        InvalidNumber,
    }

    let mut test_struct = TestStruct {
        chars: vec![b'1', b'2', b'3'], // valid integer input
        index: 0,
    };

    let result = test_struct.ignore_integer();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_integer_exponent() {
    struct TestStruct {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.index < self.chars.len() {
                let ch = self.chars[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // represents null
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Ok(0) // represents null
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.chars.len() {
                self.index += 1;
            }
        }

        fn ignore_decimal(&mut self) -> Result<()> { Ok(()) }
        fn ignore_exponent(&mut self) -> Result<()> { Ok(()) }
        
        fn ignore_integer(&mut self) -> Result<()> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(ErrorCode::InvalidNumber.into());
                    }
                }
                b'1'..=b'9' => {
                    while let b'0'..=b'9' = self.peek_or_null()? {
                        self.eat_char();
                    }
                }
                _ => {
                    return Err(ErrorCode::InvalidNumber.into());
                }
            }

            match self.peek_or_null()? {
                b'.' => self.ignore_decimal(),
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    #[derive(Debug)]
    enum ErrorCode {
        InvalidNumber,
    }

    let mut test_struct = TestStruct {
        chars: vec![b'1', b'2', b'3', b'e'], // valid integer leading to exponent
        index: 0,
    };

    let result = test_struct.ignore_integer();
    assert!(result.is_ok());
}

