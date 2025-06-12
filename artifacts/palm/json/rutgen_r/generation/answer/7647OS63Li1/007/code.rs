// Answer 0

#[test]
fn test_parse_integer_leading_zero() {
    struct Parser {
        input: Vec<u8>,
        current: usize,
    }

    impl Parser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.current < self.input.len() {
                let c = self.input[self.current];
                self.current += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<Option<u8>, ()> {
            if self.current < self.input.len() {
                Ok(Some(self.input[self.current]))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _: ()) -> () {
            // Simulated error handling
        }

        fn parse_number(&self, _: bool, _: u64) -> Result<(), ()> {
            Ok(())
        }

        fn parse_long_integer(&self, _: bool, _: u64) -> Result<u64, ()> {
            Ok(123)
        }

        fn parse_integer(&mut self, positive: bool) -> Result<(), ()> {
            let next = match self.next_char()? {
                Some(b) => b,
                None => {
                    return Err(());
                }
            };

            match next {
                b'0' => {
                    match self.peek_or_null()? {
                        b'0'..=b'9' => Err(()),
                        _ => self.parse_number(positive, 0),
                    }
                }
                c @ b'1'..=b'9' => {
                    let mut significand = (c - b'0') as u64;

                    loop {
                        match self.peek_or_null()? {
                            c @ b'0'..=b'9' => {
                                let digit = (c - b'0') as u64;
                                if significand > (u64::MAX - digit) / 10 {
                                    return Ok(());
                                }
                                self.next_char()?;
                                significand = significand * 10 + digit;
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

    let mut parser = Parser { input: vec![b'0'], current: 0 };
    let result = parser.parse_integer(true);
    assert!(result.is_err());
}

#[test]
fn test_parse_integer_valid() {
    struct Parser {
        input: Vec<u8>,
        current: usize,
    }

    impl Parser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.current < self.input.len() {
                let c = self.input[self.current];
                self.current += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<Option<u8>, ()> {
            if self.current < self.input.len() {
                Ok(Some(self.input[self.current]))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _: ()) -> () {
            // Simulated error handling
        }

        fn parse_number(&self, _: bool, _: u64) -> Result<(), ()> {
            Ok(())
        }

        fn parse_long_integer(&self, _: bool, _: u64) -> Result<u64, ()> {
            Ok(123)
        }

        fn parse_integer(&mut self, positive: bool) -> Result<(), ()> {
            let next = match self.next_char()? {
                Some(b) => b,
                None => {
                    return Err(());
                }
            };

            match next {
                b'0' => {
                    match self.peek_or_null()? {
                        b'0'..=b'9' => Err(()),
                        _ => self.parse_number(positive, 0),
                    }
                }
                c @ b'1'..=b'9' => {
                    let mut significand = (c - b'0') as u64;

                    loop {
                        match self.peek_or_null()? {
                            c @ b'0'..=b'9' => {
                                let digit = (c - b'0') as u64;
                                if significand > (u64::MAX - digit) / 10 {
                                    return Ok(());
                                }
                                self.next_char()?;
                                significand = significand * 10 + digit;
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

    let mut parser = Parser { input: vec![b'1', b'2', b'3'], current: 0 };
    let result = parser.parse_integer(true);
    assert!(result.is_ok());
}

#[test]
fn test_parse_integer_invalid_character() {
    struct Parser {
        input: Vec<u8>,
        current: usize,
    }

    impl Parser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.current < self.input.len() {
                let c = self.input[self.current];
                self.current += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<Option<u8>, ()> {
            if self.current < self.input.len() {
                Ok(Some(self.input[self.current]))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _: ()) -> () {
            // Simulated error handling
        }

        fn parse_number(&self, _: bool, _: u64) -> Result<(), ()> {
            Ok(())
        }

        fn parse_long_integer(&self, _: bool, _: u64) -> Result<u64, ()> {
            Ok(123)
        }

        fn parse_integer(&mut self, positive: bool) -> Result<(), ()> {
            let next = match self.next_char()? {
                Some(b) => b,
                None => {
                    return Err(());
                }
            };

            match next {
                b'0' => {
                    match self.peek_or_null()? {
                        b'0'..=b'9' => Err(()),
                        _ => self.parse_number(positive, 0),
                    }
                }
                c @ b'1'..=b'9' => {
                    let mut significand = (c - b'0') as u64;

                    loop {
                        match self.peek_or_null()? {
                            c @ b'0'..=b'9' => {
                                let digit = (c - b'0') as u64;
                                if significand > (u64::MAX - digit) / 10 {
                                    return Ok(());
                                }
                                self.next_char()?;
                                significand = significand * 10 + digit;
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

    let mut parser = Parser { input: vec![b'a'], current: 0 };
    let result = parser.parse_integer(true);
    assert!(result.is_err());
}

