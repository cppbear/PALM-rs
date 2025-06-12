// Answer 0

#[test]
fn test_ignore_integer_leading_zero_invalid() {
    struct TestContext {
        index: usize,
        chars: Vec<u8>,
    }

    impl TestContext {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.chars.len() {
                self.index += 1;
                Ok(self.chars[self.index - 1])
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Ok(0)
            }
        }

        fn error(&self, _: ErrorCode) -> Result<(), &'static str> {
            Err("Invalid Number")
        }

        fn eat_char(&mut self) {
            if self.index < self.chars.len() {
                self.index += 1;
            }
        }

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            // Simulated ignore_decimal for testing
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            // Simulated ignore_exponent for testing
            Ok(())
        }
    }

    let mut context = TestContext {
        index: 0,
        chars: vec![b'0', b'1'],
    };

    let result = context.ignore_integer();
    assert!(result.is_err());
}
    
#[test]
fn test_ignore_integer_valid() {
    struct TestContext {
        index: usize,
        chars: Vec<u8>,
    }

    impl TestContext {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.chars.len() {
                self.index += 1;
                Ok(self.chars[self.index - 1])
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Ok(0)
            }
        }

        fn error(&self, _: ErrorCode) -> Result<(), &'static str> {
            Err("Invalid Number")
        }

        fn eat_char(&mut self) {
            if self.index < self.chars.len() {
                self.index += 1;
            }
        }

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            Ok(())
        }
    }

    let mut context = TestContext {
        index: 0,
        chars: vec![b'1', b'2', b'3', b'.'],
    };

    let result = context.ignore_integer();
    assert!(result.is_ok());
}

