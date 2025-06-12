// Answer 0

#[test]
fn test_ignore_integer_valid_single_digit() {
    struct TestStruct {
        input: &'static [u8],
        index: usize,
    }

    impl TestStruct {
        fn next_char_or_null(&mut self) -> Result<u8, ErrorCode> {
            if self.index >= self.input.len() {
                return Ok(0); // Return null character
            }
            let ch = self.input[self.index];
            self.index += 1;
            Ok(ch)
        }

        fn peek_or_null(&self) -> Result<u8, ErrorCode> {
            if self.index >= self.input.len() {
                return Ok(0);
            }
            Ok(self.input[self.index])
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _code: ErrorCode) -> Result<(), ErrorCode> {
            Err(ErrorCode::InvalidNumber)
        }

        fn ignore_decimal(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }
    }

    let mut test_instance = TestStruct { input: b"3", index: 0 };
    let result = test_instance.ignore_integer();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_integer_valid_multi_digit() {
    struct TestStruct {
        input: &'static [u8],
        index: usize,
    }

    impl TestStruct {
        fn next_char_or_null(&mut self) -> Result<u8, ErrorCode> {
            if self.index >= self.input.len() {
                return Ok(0);
            }
            let ch = self.input[self.index];
            self.index += 1;
            Ok(ch)
        }

        fn peek_or_null(&self) -> Result<u8, ErrorCode> {
            if self.index >= self.input.len() {
                return Ok(0);
            }
            Ok(self.input[self.index])
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _code: ErrorCode) -> Result<(), ErrorCode> {
            Err(ErrorCode::InvalidNumber)
        }

        fn ignore_decimal(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }
    }

    let mut test_instance = TestStruct { input: b"123456", index: 0 };
    let result = test_instance.ignore_integer();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_integer_leading_zero() {
    struct TestStruct {
        input: &'static [u8],
        index: usize,
    }

    impl TestStruct {
        fn next_char_or_null(&mut self) -> Result<u8, ErrorCode> {
            if self.index >= self.input.len() {
                return Ok(0);
            }
            let ch = self.input[self.index];
            self.index += 1;
            Ok(ch)
        }

        fn peek_or_null(&self) -> Result<u8, ErrorCode> {
            if self.index >= self.input.len() {
                return Ok(0);
            }
            Ok(self.input[self.index])
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _code: ErrorCode) -> Result<(), ErrorCode> {
            Err(ErrorCode::InvalidNumber)
        }

        fn ignore_decimal(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }
    }

    let mut test_instance = TestStruct { input: b"00", index: 0 };
    let result = test_instance.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_invalid_character() {
    struct TestStruct {
        input: &'static [u8],
        index: usize,
    }

    impl TestStruct {
        fn next_char_or_null(&mut self) -> Result<u8, ErrorCode> {
            if self.index >= self.input.len() {
                return Ok(0);
            }
            let ch = self.input[self.index];
            self.index += 1;
            Ok(ch)
        }

        fn peek_or_null(&self) -> Result<u8, ErrorCode> {
            if self.index >= self.input.len() {
                return Ok(0);
            }
            Ok(self.input[self.index])
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn error(&self, _code: ErrorCode) -> Result<(), ErrorCode> {
            Err(ErrorCode::InvalidNumber)
        }

        fn ignore_decimal(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }
    }

    let mut test_instance = TestStruct { input: b"a", index: 0 };
    let result = test_instance.ignore_integer();
    assert!(result.is_err());
}

