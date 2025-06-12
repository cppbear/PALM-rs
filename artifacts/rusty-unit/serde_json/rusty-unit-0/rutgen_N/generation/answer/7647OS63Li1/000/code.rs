// Answer 0

#[derive(Debug)]
struct Parser {
    input: Vec<u8>,
    position: usize,
}

#[derive(Debug)]
enum ParserNumber {
    U64(u64),
    F64(f64),
}

#[derive(Debug)]
enum ErrorCode {
    EofWhileParsingValue,
    InvalidNumber,
}

#[derive(Debug)]
struct ParserError(ErrorCode);

impl Parser {
    fn new(input: Vec<u8>) -> Self {
        Self { input, position: 0 }
    }

    fn next_char(&mut self) -> Option<u8> {
        if self.position < self.input.len() {
            let c = self.input[self.position];
            self.position += 1;
            Some(c)
        } else {
            None
        }
    }

    fn peek_or_null(&mut self) -> Option<u8> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn eat_char(&mut self) {
        self.position += 1;
    }

    fn parse_number(&self, positive: bool, significand: u64) -> Result<ParserNumber, ParserError> {
        if positive {
            Ok(ParserNumber::U64(significand))
        } else {
            Ok(ParserNumber::U64(!significand + 1)) // Example handling for negative numbers
        }
    }

    fn parse_long_integer(&self, positive: bool, significand: u64) -> Result<ParserNumber, ParserError> {
        // Simulate parsing long integer, returning as f64 for large numbers.
        if positive {
            Ok(ParserNumber::F64(significand as f64))
        } else {
            Ok(ParserNumber::F64(-significand as f64)) // Example handling for negative numbers
        }
    }

    fn error(&self, code: ErrorCode) -> ParserError {
        ParserError(code)
    }

    fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber, ParserError> {
        let next = match self.next_char() {
            Some(b) => b,
            None => {
                return Err(self.error(ErrorCode::EofWhileParsingValue));
            }
        };

        match next {
            b'0' => {
                match self.peek_or_null() {
                    Some(c @ b'0'..=b'9') => Err(self.error(ErrorCode::InvalidNumber)),
                    _ => self.parse_number(positive, 0),
                }
            }
            c @ b'1'..=b'9' => {
                let mut significand = (c - b'0') as u64;

                loop {
                    match self.peek_or_null() {
                        Some(c @ b'0'..=b'9') => {
                            let digit = (c - b'0') as u64;

                            if significand > (u64::MAX - digit) / 10 {
                                return Ok(ParserNumber::F64(self.parse_long_integer(positive, significand)?));
                            }

                            self.eat_char();
                            significand = significand * 10 + digit;
                        }
                        _ => {
                            return self.parse_number(positive, significand);
                        }
                    }
                }
            }
            _ => Err(self.error(ErrorCode::InvalidNumber)),
        }
    }
}

#[test]
fn test_parse_integer_zero() {
    let mut parser = Parser::new(vec![b'0']);
    assert_eq!(parser.parse_integer(true), Ok(ParserNumber::U64(0)));
}

#[test]
fn test_parse_integer_leading_zero() {
    let mut parser = Parser::new(vec![b'0', b'1']);
    assert_eq!(parser.parse_integer(true), Err(ParserError(ErrorCode::InvalidNumber)));
}

#[test]
fn test_parse_integer_single_digit() {
    let mut parser = Parser::new(vec![b'5']);
    assert_eq!(parser.parse_integer(true), Ok(ParserNumber::U64(5)));
}

#[test]
fn test_parse_integer_multiple_digits() {
    let mut parser = Parser::new(vec![b'1', b'2', b'3']);
    assert_eq!(parser.parse_integer(true), Ok(ParserNumber::U64(123)));
}

#[test]
fn test_parse_integer_invalid_character() {
    let mut parser = Parser::new(vec![b'a']);
    assert_eq!(parser.parse_integer(true), Err(ParserError(ErrorCode::InvalidNumber)));
}

#[test]
fn test_parse_integer_eof() {
    let mut parser = Parser::new(vec![]);
    assert_eq!(parser.parse_integer(true), Err(ParserError(ErrorCode::EofWhileParsingValue)));
}

