// Answer 0

#[test]
fn test_ignore_exponent_with_valid_exponent() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: &[u8]) -> Self {
            MockParser { input: input.to_vec(), index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.eat_char();
                Ok(ch)
            } else {
                Err(())
            }
        }

        fn error(&self, _: ErrorCode) -> () { () }
    }

    let mut parser = MockParser::new(b"e+1");
    let result = parser.ignore_exponent();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_ignore_exponent_with_missing_digit() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: &[u8]) -> Self {
            MockParser { input: input.to_vec(), index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.eat_char();
                Ok(ch)
            } else {
                Err(())
            }
        }

        fn error(&self, _: ErrorCode) -> () { () }
    }

    let mut parser = MockParser::new(b"e+");
    let _ = parser.ignore_exponent();
}

#[test]
fn test_ignore_exponent_with_invalid_character() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(input: &[u8]) -> Self {
            MockParser { input: input.to_vec(), index: 0 }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.eat_char();
                Ok(ch)
            } else {
                Err(())
            }
        }

        fn error(&self, _: ErrorCode) -> () { () }
    }

    let mut parser = MockParser::new(b"e+z");
    let result = parser.ignore_exponent();
    assert_eq!(result, Err(parser.error(ErrorCode::InvalidNumber)));
}

