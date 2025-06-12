// Answer 0

fn test_ignore_exponent_valid() {
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

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Err("End of input")
            }
        }
        
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.input.len() {
                let val = self.input[self.pos];
                self.eat_char();
                Ok(val)
            } else {
                Err("End of input")
            }
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            self.eat_char();

            match self.peek_or_null()? {
                b'+' | b'-' => self.eat_char(),
                _ => {}
            }

            match self.next_char_or_null()? {
                b'0'..=b'9' => {}
                _ => {
                    return Err("Invalid number");
                }
            }

            while let Ok(b'0'..=b'9') = self.peek_or_null() {
                self.eat_char();
            }

            Ok(())
        }
    }

    let mut parser = MockParser::new(b"e+123".to_vec());
    let result = parser.ignore_exponent();
    assert!(result.is_ok());
}

fn test_ignore_exponent_invalid() {
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

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Err("End of input")
            }
        }
        
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.pos < self.input.len() {
                let val = self.input[self.pos];
                self.eat_char();
                Ok(val)
            } else {
                Err("End of input")
            }
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            self.eat_char();

            match self.peek_or_null()? {
                b'+' | b'-' => self.eat_char(),
                _ => {}
            }

            match self.next_char_or_null()? {
                b'0'..=b'9' => {}
                _ => {
                    return Err("Invalid number");
                }
            }

            while let Ok(b'0'..=b'9') = self.peek_or_null() {
                self.eat_char();
            }

            Ok(())
        }
    }

    let mut parser = MockParser::new(b"e+".to_vec());
    let result = parser.ignore_exponent();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid number");
}

