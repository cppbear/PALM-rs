// Answer 0

fn test_ignore_decimal() {
    struct MockParser {
        input: Vec<u8>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Err(())
            }
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            ()
        }

        fn ignore_exponent(&mut self) -> Result<(), ()> {
            if matches!(self.peek_or_null(), Ok(b'e') | Ok(b'E')) {
                self.eat_char();
                Ok(())
            } else {
                Ok(())
            }
        }

        fn ignore_decimal(&mut self) -> Result<(), ()> {
            self.eat_char();
            let mut at_least_one_digit = false;
            while let Ok(b'0'..=b'9') = self.peek_or_null() {
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

    enum ErrorCode {
        InvalidNumber,
    }

    // Test case where there are no digits after calling eat_char
    let mut parser_no_digits = MockParser::new(vec![b'.']);
    assert_eq!(parser_no_digits.ignore_decimal(), Err(())); // should return Err due to no digits

    // Test case where there is at least one digit and an exponent
    let mut parser_with_exponent = MockParser::new(vec![b'1', b'2', b'e']);
    assert_eq!(parser_with_exponent.ignore_decimal(), Ok(())); // should succeed and ignore exponent

    // Test case where there is at least one digit and no exponent
    let mut parser_with_no_exponent = MockParser::new(vec![b'3', b'4']);
    assert_eq!(parser_with_no_exponent.ignore_decimal(), Ok(())); // should succeed without exponent

    // Test case where there are multiple digits and then an invalid character
    let mut parser_invalid = MockParser::new(vec![b'5', b'6', b'a']);
    assert_eq!(parser_invalid.ignore_decimal(), Ok(())); // should succeed, ignores invalid char after digits

    // Test case where input is empty
    let mut parser_empty = MockParser::new(vec![]);
    assert_eq!(parser_empty.ignore_decimal(), Err(())); // should return Err due to empty input
}

