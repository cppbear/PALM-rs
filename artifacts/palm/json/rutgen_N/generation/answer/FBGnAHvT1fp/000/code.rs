// Answer 0

#[test]
fn test_ignore_decimal_valid() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            TestReader { input, position: 0 }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(b'\0')
            }
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            panic!("Invalid number error");
        }

        fn ignore_exponent(&mut self) -> Result<(), ()> {
            self.eat_char(); // consume the 'e' or 'E'
            if self.position < self.input.len() {
                self.eat_char(); // consume the next character
            }
            Ok(())
        }
        
        fn ignore_decimal(&mut self) -> Result<(), ()> {
            self.eat_char();

            let mut at_least_one_digit = false;
            while let b'0'..=b'9' = self.peek_or_null().unwrap() {
                self.eat_char();
                at_least_one_digit = true;
            }

            if !at_least_one_digit {
                return Err(self.peek_error(ErrorCode::InvalidNumber));
            }

            match self.peek_or_null().unwrap() {
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut reader = TestReader::new(b"12345e6".to_vec());
    let result = reader.ignore_decimal();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_ignore_decimal_no_digits() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            TestReader { input, position: 0 }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(b'\0')
            }
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            panic!("Invalid number error");
        }

        fn ignore_exponent(&mut self) -> Result<(), ()> {
            self.eat_char();
            if self.position < self.input.len() {
                self.eat_char();
            }
            Ok(())
        }

        fn ignore_decimal(&mut self) -> Result<(), ()> {
            self.eat_char();

            let mut at_least_one_digit = false;
            while let b'0'..=b'9' = self.peek_or_null().unwrap() {
                self.eat_char();
                at_least_one_digit = true;
            }

            if !at_least_one_digit {
                return Err(self.peek_error(ErrorCode::InvalidNumber));
            }

            match self.peek_or_null().unwrap() {
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut reader = TestReader::new(b".".to_vec());
    let _ = reader.ignore_decimal();
}

