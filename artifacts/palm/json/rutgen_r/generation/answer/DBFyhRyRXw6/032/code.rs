// Answer 0

#[derive(Debug)]
struct MockParser {
    chars: Vec<u8>,
    pos: usize,
}

impl MockParser {
    fn new(chars: Vec<u8>) -> Self {
        Self { chars, pos: 0 }
    }

    fn eat_char(&mut self) {
        self.pos += 1;
    }

    fn peek_or_null(&self) -> Result<u8, ()> {
        if self.pos < self.chars.len() {
            Ok(self.chars[self.pos])
        } else {
            Err(())
        }
    }

    fn next_char(&mut self) -> Result<Option<u8>, ()> {
        if self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            self.eat_char();
            Ok(Some(c))
        } else {
            Ok(None)
        }
    }

    fn error(&self, _code: &'static str) -> () {
        ()
    }

    fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64, ()> {
        Ok(0.0) // Simplified for testing purposes
    }

    fn parse_exponent(
        &mut self,
        positive: bool,
        significand: u64,
        starting_exp: i32,
    ) -> Result<f64, ()> {
        self.eat_char();

        let positive_exp = match self.peek_or_null() {
            Ok(b'+') => {
                self.eat_char();
                true
            }
            Ok(b'-') => {
                self.eat_char();
                false
            }
            _ => true,
        };

        let next = match self.next_char()? {
            Some(b) => b,
            None => {
                return Err(self.error("EofWhileParsingValue"));
            }
        };

        let mut exp = match next {
            c @ b'0'..=b'9' => (c - b'0') as i32,
            _ => {
                return Err(self.error("InvalidNumber"));
            }
        };

        while let Ok(c) = self.peek_or_null() {
            if c >= b'0' && c <= b'9' {
                self.eat_char();
                let digit = (c - b'0') as i32;

                if exp.checked_mul(10).and_then(|v| v.checked_add(digit)).is_none() {
                    return Err(self.error("Overflow"));
                }

                exp = exp * 10 + digit;
            } else {
                break;
            }
        }

        let final_exp = if positive_exp {
            starting_exp.saturating_add(exp)
        } else {
            starting_exp.saturating_sub(exp)
        };

        self.f64_from_parts(positive, significand, final_exp)
    }
}

#[test]
fn test_parse_exponent_positive() {
    let mut parser = MockParser::new(vec![b'e', b'+', b'2']);
    let result = parser.parse_exponent(true, 1, 0);
    assert_eq!(result, Ok(0.0));
}

#[test]
fn test_parse_exponent_negative() {
    let mut parser = MockParser::new(vec![b'e', b'-', b'3']);
    let result = parser.parse_exponent(false, 1, 5);
    assert_eq!(result, Ok(0.0));
}

#[test]
fn test_parse_exponent_invalid_char() {
    let mut parser = MockParser::new(vec![b'e', b'*']);
    let result = parser.parse_exponent(true, 1, 0);
    assert!(result.is_err());
}

#[test]
fn test_parse_exponent_eof() {
    let mut parser = MockParser::new(vec![b'e']);
    let result = parser.parse_exponent(true, 1, 0);
    assert!(result.is_err());
}

#[test]
fn test_parse_exponent_overflow() {
    let mut parser = MockParser::new(vec![b'e', b'+', b'1', b'0', b'0', b'0', b'0', b'0', b'0']);
    let result = parser.parse_exponent(true, 1, i32::MAX);
    assert!(result.is_err());
}

