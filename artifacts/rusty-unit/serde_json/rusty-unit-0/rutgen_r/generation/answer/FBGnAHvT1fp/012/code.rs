// Answer 0

#[test]
fn test_ignore_decimal_no_digit() {
    struct Parser {
        input: Vec<u8>,
        index: usize,
    }

    impl Parser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            // Simplified error handling for the test
            panic!("Invalid number");
        }

        fn ignore_exponent(&mut self) -> Result<(), ()> {
            // Not tested here, just a placeholder
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

    enum ErrorCode {
        InvalidNumber,
    }

    let mut parser = Parser {
        input: b"abc".to_vec(),
        index: 0,
    };

    let result = parser.ignore_decimal();
    assert!(result.is_err());
}

#[test]
fn test_ignore_decimal_empty_input() {
    struct Parser {
        input: Vec<u8>,
        index: usize,
    }

    impl Parser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            // Simplified error handling for the test
            panic!("Invalid number");
        }

        fn ignore_exponent(&mut self) -> Result<(), ()> {
            // Not tested here, just a placeholder
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

    enum ErrorCode {
        InvalidNumber,
    }

    let mut parser = Parser {
        input: Vec::new(),  // Empty input
        index: 0,
    };

    let result = parser.ignore_decimal();
    assert!(result.is_err());
}

