// Answer 0

#[test]
fn test_ignore_integer_leading_zero_with_subsequent_digit() {
    struct Mock {
        data: Vec<u8>,
        index: usize,
    }

    impl Mock {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Ok(result)
            } else {
                Ok(b'\0') // Simulating null character at the end
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Ok(b'\0') // Simulating null character at the end
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            // Dummy implementation for the sake of test
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            // Dummy implementation for the sake of test
            Ok(())
        }

        fn error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::InvalidNumber.into())
        }

        fn peek_error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::InvalidNumber.into())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(self.peek_error(ErrorCode::InvalidNumber));
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

    let mut mock = Mock {
        data: vec![b'0', b'1'],
        index: 0,
    };

    assert_eq!(mock.ignore_integer(), Err(ErrorCode::InvalidNumber.into()));
}

#[test]
fn test_ignore_integer_valid_number() {
    struct Mock {
        data: Vec<u8>,
        index: usize,
    }

    impl Mock {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Ok(result)
            } else {
                Ok(b'\0')
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Ok(b'\0')
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            Ok(())
        }

        fn error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::InvalidNumber.into())
        }

        fn peek_error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::InvalidNumber.into())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(self.peek_error(ErrorCode::InvalidNumber));
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

    let mut mock = Mock {
        data: vec![b'1', b'2', b'3'],
        index: 0,
    };

    assert_eq!(mock.ignore_integer(), Ok(()));
}

#[test]
fn test_ignore_integer_invalid_character() {
    struct Mock {
        data: Vec<u8>,
        index: usize,
    }

    impl Mock {
        fn next_char_or_null(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Ok(result)
            } else {
                Ok(b'\0')
            }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Ok(b'\0')
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            Ok(())
        }

        fn error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::InvalidNumber.into())
        }

        fn peek_error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::InvalidNumber.into())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(self.peek_error(ErrorCode::InvalidNumber));
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

    let mut mock = Mock {
        data: vec![b'a'],
        index: 0,
    };

    assert_eq!(mock.ignore_integer(), Err(ErrorCode::InvalidNumber.into()));
}

