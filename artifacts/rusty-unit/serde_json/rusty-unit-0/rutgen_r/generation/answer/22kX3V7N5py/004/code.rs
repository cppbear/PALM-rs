// Answer 0

#[test]
fn test_parse_any_signed_number_positive() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_any_number(&mut self, positive: bool) -> Result<ParserNumber, ()> {
            if positive {
                if self.index < self.input.len() {
                    let value = self.input[self.index];
                    self.index += 1;
                    Ok(ParserNumber::new(value as i64)) // Assuming ParserNumber::new can create from i64
                } else {
                    Err(())
                }
            } else {
                Ok(ParserNumber::new(-1))
            }
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            // No-op for this mock
        }

        fn fix_position(&self, _: ()) -> () {
            // No-op for this mock
        }
    }

    let mut parser = MockParser {
        input: vec![b'5'],
        index: 0,
    };

    let result = parser.parse_any_signed_number();
    assert!(result.is_ok());
}

#[test]
fn test_parse_any_signed_number_negative() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_any_number(&mut self, positive: bool) -> Result<ParserNumber, ()> {
            if !positive {
                return Ok(ParserNumber::new(-1));
            }
            Err(())
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            // No-op for this mock
        }

        fn fix_position(&self, _: ()) -> () {
            // No-op for this mock
        }
    }

    let mut parser = MockParser {
        input: vec![b'-', b'2'],
        index: 0,
    };

    let result = parser.parse_any_signed_number();
    assert!(result.is_ok());
}

#[test]
fn test_parse_any_signed_number_invalid_character() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_any_number(&mut self, _: bool) -> Result<ParserNumber, ()> {
            Err(())
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            // No-op for this mock
        }

        fn fix_position(&self, err: ()) -> Result<(), ()> {
            Err(err)
        }
    }

    let mut parser = MockParser {
        input: vec![b'a'],
        index: 0,
    };

    let result = parser.parse_any_signed_number();
    assert!(result.is_err());
}

