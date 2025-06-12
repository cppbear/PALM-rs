// Answer 0

#[test]
fn test_parse_integer_leading_zero() {
    struct TestParser<'a> {
        input: &'a [u8],
        index: usize,
    }

    impl<'a> TestParser<'a> {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let c = self.input[self.index];
                self.index += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn parse_number(&self, _positive: bool, significand: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(significand))
        }

        fn error(&self, _code: ErrorCode) -> () {
            ()
        }

        fn parse_long_integer(&self, _positive: bool, _significand: u64) -> Result<u64, ()> {
            Ok(42)
        }

        fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber, ()> {
            let next = match self.next_char()? {
                Some(b) => b,
                None => {
                    return Err(self.error(ErrorCode::EofWhileParsingValue));
                }
            };

            match next {
                b'0' => {
                    match self.peek_or_null() {
                        Ok(b'0'..=b'9') => Err(self.error(ErrorCode::InvalidNumber)),
                        _ => self.parse_number(positive, 0),
                    }
                }
                _ => Err(self.error(ErrorCode::InvalidNumber)),
            }
        }
    }

    let mut parser = TestParser { input: &[b'0'], index: 0 };
    let result = parser.parse_integer(true);
    assert_eq!(result, Ok(ParserNumber::U64(0)));
}

#[test]
fn test_parse_integer_invalid_number() {
    struct TestParser<'a> {
        input: &'a [u8],
        index: usize,
    }

    impl<'a> TestParser<'a> {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let c = self.input[self.index];
                self.index += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn error(&self, _code: ErrorCode) -> () {
            ()
        }

        fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber, ()> {
            let next = match self.next_char()? {
                Some(b) => b,
                None => {
                    return Err(self.error(ErrorCode::EofWhileParsingValue));
                }
            };

            match next {
                b'0' => {
                    match self.peek_or_null() {
                        Ok(b'0'..=b'9') => return Err(self.error(ErrorCode::InvalidNumber)),
                        _ => return Ok(ParserNumber::U64(0)),
                    }
                }
                c if (b'1'..=b'9').contains(&c) => Ok(ParserNumber::U64(1)),
                _ => Err(self.error(ErrorCode::InvalidNumber)),
            }
        }
    }

    let mut parser = TestParser { input: &[b'0', b'0'], index: 0 };
    let result = parser.parse_integer(true);
    assert_eq!(result, Err(()));
}

#[test]
fn test_parse_integer_end_of_input() {
    struct TestParser<'a> {
        input: &'a [u8],
        index: usize,
    }

    impl<'a> TestParser<'a> {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let c = self.input[self.index];
                self.index += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: ErrorCode) -> () {
            ()
        }

        fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber, ()> {
            let next = match self.next_char()? {
                Some(b) => b,
                None => {
                    return Err(self.error(ErrorCode::EofWhileParsingValue));
                }
            };

            match next {
                _ => Err(self.error(ErrorCode::InvalidNumber)),
            }
        }
    }

    let mut parser = TestParser { input: &[], index: 0 };
    let result = parser.parse_integer(true);
    assert_eq!(result, Err(()));
}

