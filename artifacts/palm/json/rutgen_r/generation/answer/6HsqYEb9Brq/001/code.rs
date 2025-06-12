// Answer 0

#[test]
fn test_ignore_integer_invalid_number_leading_zero() {
    struct TestStruct {
        chars: Vec<u8>,
        pos: usize,
    }

    impl TestStruct {
        fn new(chars: Vec<u8>) -> Self {
            Self { chars, pos: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                let ch = self.chars[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                Ok(self.chars[self.pos])
            } else {
                Ok(0)
            }
        }

        fn error(&self, _code: ErrorCode) -> Result<(), &'static str> {
            Err("Invalid number")
        }

        fn eat_char(&mut self) {}

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            Ok(())
        }
    }

    let mut instance = TestStruct::new(vec![b'0', b'0']);
    let result = instance.ignore_integer();
    assert_eq!(result, Err("Invalid number"));
}

#[test]
fn test_ignore_integer_invalid_character() {
    struct TestStruct {
        chars: Vec<u8>,
        pos: usize,
    }

    impl TestStruct {
        fn new(chars: Vec<u8>) -> Self {
            Self { chars, pos: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                let ch = self.chars[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Ok(0)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.chars.len() {
                Ok(self.chars[self.pos])
            } else {
                Ok(0)
            }
        }

        fn error(&self, _code: ErrorCode) -> Result<(), &'static str> {
            Err("Invalid number")
        }

        fn eat_char(&mut self) {}

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            Ok(())
        }
    }

    let mut instance = TestStruct::new(vec![b'a']); // an invalid character
    let result = instance.ignore_integer();
    assert_eq!(result, Err("Invalid number"));
}

