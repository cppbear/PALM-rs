// Answer 0

fn test_ignore_decimal_valid_number() {
    struct Parser {
        input: Vec<u8>,
        index: usize,
    }

    impl Parser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0) // Assuming null is represented by 0
            }
        }

        fn peek_error(&self, _error: &str) -> Result<(), &'static str> {
            Err("Invalid Number")
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            self.eat_char(); // Simulates ignoring exponent
            Ok(())
        }

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            self.eat_char();

            let mut at_least_one_digit = false;
            while let Ok(b'0'..=b'9') = self.peek_or_null() {
                self.eat_char();
                at_least_one_digit = true;
            }

            if !at_least_one_digit {
                return Err(self.peek_error("InvalidNumber").unwrap_err());
            }

            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut parser = Parser {
        input: b"12345e".to_vec(),
        index: 0,
    };
    let result = parser.ignore_decimal();
    assert_eq!(result, Ok(()));
}

fn test_ignore_decimal_no_digits() {
    struct Parser {
        input: Vec<u8>,
        index: usize,
    }

    impl Parser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn peek_error(&self, _error: &str) -> Result<(), &'static str> {
            Err("Invalid Number")
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            self.eat_char();
            Ok(())
        }

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            self.eat_char();

            let mut at_least_one_digit = false;
            while let Ok(b'0'..=b'9') = self.peek_or_null() {
                self.eat_char();
                at_least_one_digit = true;
            }

            if !at_least_one_digit {
                return Err(self.peek_error("InvalidNumber").unwrap_err());
            }

            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut parser = Parser {
        input: b"abc".to_vec(),
        index: 0,
    };
    let result = parser.ignore_decimal();
    assert_eq!(result, Err("Invalid Number"));
}

fn test_ignore_decimal_with_exponent() {
    struct Parser {
        input: Vec<u8>,
        index: usize,
    }

    impl Parser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Ok(0)
            }
        }

        fn peek_error(&self, _error: &str) -> Result<(), &'static str> {
            Err("Invalid Number")
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            self.eat_char();
            Ok(())
        }

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            self.eat_char();

            let mut at_least_one_digit = false;
            while let Ok(b'0'..=b'9') = self.peek_or_null() {
                self.eat_char();
                at_least_one_digit = true;
            }

            if !at_least_one_digit {
                return Err(self.peek_error("InvalidNumber").unwrap_err());
            }

            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut parser = Parser {
        input: b"4567E09".to_vec(),
        index: 0,
    };
    let result = parser.ignore_decimal();
    assert_eq!(result, Ok(()));
}

