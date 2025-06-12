// Answer 0

fn test_ignore_decimal_valid() -> Result<()> {
    struct MockParser {
        buffer: Vec<u8>,
        current_index: usize,
    }

    impl MockParser {
        fn new(buffer: Vec<u8>) -> Self {
            MockParser { buffer, current_index: 0 }
        }

        fn eat_char(&mut self) {
            if self.current_index < self.buffer.len() {
                self.current_index += 1;
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.current_index < self.buffer.len() {
                Ok(self.buffer[self.current_index])
            } else {
                Err(())
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            // Simplified version of the error handling method
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            // Simplified version of the method that would handle the exponent
            Ok(())
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            self.eat_char();

            let mut at_least_one_digit = false;
            while let b'0'..=b'9' = self.peek_or_null()? {
                self.eat_char();
                at_least_one_digit = true;
            }

            if !at_least_one_digit {
                return Err(self.peek_error(ErrorCode::InvalidNumber));
            }

            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut parser = MockParser::new(vec![b'2', b'3', b'4', b'e']);
    assert_eq!(parser.ignore_decimal(), Ok(()));

    let mut parser_invalid = MockParser::new(vec![b'e']);
    assert_eq!(parser_invalid.ignore_decimal(), Err(parser_invalid.peek_error(ErrorCode::InvalidNumber)));

    Ok(())
}

fn test_ignore_decimal_invalid_no_digit() -> Result<()> {
    struct MockParser {
        buffer: Vec<u8>,
        current_index: usize,
    }

    impl MockParser {
        fn new(buffer: Vec<u8>) -> Self {
            MockParser { buffer, current_index: 0 }
        }

        fn eat_char(&mut self) {
            if self.current_index < self.buffer.len() {
                self.current_index += 1;
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.current_index < self.buffer.len() {
                Ok(self.buffer[self.current_index])
            } else {
                Err(())
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            // Simplified version of the error handling method
        }

        fn ignore_exponent(&mut self) -> Result<()> {
            // Simplified version of the method that would handle the exponent
            Ok(())
        }

        fn ignore_decimal(&mut self) -> Result<()> {
            self.eat_char();

            let mut at_least_one_digit = false;
            while let b'0'..=b'9' = self.peek_or_null()? {
                self.eat_char();
                at_least_one_digit = true;
            }

            if !at_least_one_digit {
                return Err(self.peek_error(ErrorCode::InvalidNumber));
            }

            match self.peek_or_null() {
                Ok(b'e') | Ok(b'E') => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut parser = MockParser::new(vec![b'a']);
    assert_eq!(parser.ignore_decimal(), Err(parser.peek_error(ErrorCode::InvalidNumber)));

    Ok(())
}

