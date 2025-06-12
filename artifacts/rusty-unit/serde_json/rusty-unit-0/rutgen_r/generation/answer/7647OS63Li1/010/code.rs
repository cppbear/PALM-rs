// Answer 0

#[test]
fn test_parse_integer_zero_with_error_on_leading_zero() {
    struct Parser {
        input: Vec<u8>,
        index: usize,
    }

    impl Parser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn parse_number(&self, positive: bool, significand: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(significand))
        }

        fn parse_long_integer(&self, _positive: bool, _significand: u64) -> Result<u64, ()> {
            Ok(123) // Dummy value
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Error handling
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber, ()> {
            let next = self.next_char().unwrap()?;
            match next {
                b'0' => {
                    match self.peek_or_null().unwrap()? {
                        b'0'..=b'9' => Err(()), // This simulates an error
                        _ => self.parse_number(positive, 0),
                    }
                }
                c @ b'1'..=b'9' => {
                    let mut significand = (c - b'0') as u64;
                    loop {
                        match self.peek_or_null().unwrap()? {
                            c @ b'0'..=b'9' => {
                                let digit = (c - b'0') as u64;
                                significand = significand * 10 + digit;
                                self.eat_char();
                            }
                            _ => {
                                return self.parse_number(positive, significand);
                            }
                        }
                    }
                }
                _ => Err(()),
            }
        }
    }

    let mut parser = Parser {
        input: vec![b'0'], // Leading zero input
        index: 0,
    };

    assert_eq!(parser.parse_integer(true), Err(()));
}

#[test]
fn test_parse_integer_positive_number() {
    struct Parser {
        input: Vec<u8>,
        index: usize,
    }

    impl Parser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn parse_number(&self, positive: bool, significand: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(significand))
        }

        fn parse_long_integer(&self, _positive: bool, _significand: u64) -> Result<u64, ()> {
            Ok(123) // Dummy value
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Error handling
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber, ()> {
            let next = self.next_char().unwrap()?;
            match next {
                b'0' => {
                    match self.peek_or_null().unwrap()? {
                        b'0'..=b'9' => Err(()), // This simulates an error
                        _ => self.parse_number(positive, 0),
                    }
                }
                c @ b'1'..=b'9' => {
                    let mut significand = (c - b'0') as u64;
                    loop {
                        match self.peek_or_null().unwrap()? {
                            c @ b'0'..=b'9' => {
                                let digit = (c - b'0') as u64;
                                significand = significand * 10 + digit;
                                self.eat_char();
                            }
                            _ => {
                                return self.parse_number(positive, significand);
                            }
                        }
                    }
                }
                _ => Err(()),
            }
        }
    }

    let mut parser = Parser {
        input: vec![b'1', b'2', b'3'], // Input for valid number "123"
        index: 0,
    };

    assert_eq!(parser.parse_integer(true), Ok(ParserNumber::U64(123))); // Assuming parse_number returns Ok(ParserNumber::U64(123))
}

