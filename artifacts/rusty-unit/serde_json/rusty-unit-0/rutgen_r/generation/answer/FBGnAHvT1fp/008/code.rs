// Answer 0

fn test_ignore_decimal_no_digits() {
    struct TestReader {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(chars: Vec<u8>) -> Self {
            TestReader { chars, index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Err(())
            }
        }

        fn peek_error(&self, _code: ()) -> Result<u8, ()> {
            Err(())
        }

        fn ignore_exponent(&mut self) -> Result<(), ()> {
            self.eat_char();
            Ok(())
        }
    }

    impl TestReader {
        fn ignore_decimal(&mut self) -> Result<(), ()> {
            self.eat_char();

            let mut at_least_one_digit = false;
            while let b'0'..=b'9' = self.peek_or_null().unwrap_or(0) {
                self.eat_char();
                at_least_one_digit = true;
            }

            if !at_least_one_digit {
                return Err(self.peek_error(()).unwrap_err());
            }

            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut reader = TestReader::new(b"abc".to_vec());
    let result = reader.ignore_decimal();
    assert!(result.is_err());
}

fn test_ignore_decimal_with_digits() {
    struct TestReader {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(chars: Vec<u8>) -> Self {
            TestReader { chars, index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.chars.len() {
                Ok(self.chars[self.index])
            } else {
                Err(())
            }
        }

        fn peek_error(&self, _code: ()) -> Result<u8, ()> {
            Err(())
        }

        fn ignore_exponent(&mut self) -> Result<(), ()> {
            self.eat_char();
            Ok(())
        }
    }

    impl TestReader {
        fn ignore_decimal(&mut self) -> Result<(), ()> {
            self.eat_char();

            let mut at_least_one_digit = false;
            while let b'0'..=b'9' = self.peek_or_null().unwrap_or(0) {
                self.eat_char();
                at_least_one_digit = true;
            }

            if !at_least_one_digit {
                return Err(self.peek_error(()).unwrap_err());
            }

            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut reader = TestReader::new(b"123".to_vec());
    let result = reader.ignore_decimal();
    assert!(result.is_ok());
}

