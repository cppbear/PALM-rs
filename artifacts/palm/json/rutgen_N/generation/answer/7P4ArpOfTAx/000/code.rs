// Answer 0

#[derive(Debug)]
struct MockParser {
    position: usize,
    input: Vec<u8>,
}

impl MockParser {
    fn new(input: &[u8]) -> Self {
        Self {
            position: 0,
            input: input.to_vec(),
        }
    }

    fn peek_or_null(&self) -> Result<u8, &str> {
        if self.position < self.input.len() {
            Ok(self.input[self.position])
        } else {
            Ok(0)
        }
    }

    fn eat_char(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
        }
    }

    fn parse_exponent(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &str> {
        Ok(0.0) // Stub for the example
    }

    fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, &str> {
        Ok(0.0) // Stub for the example
    }
}

impl MockParser {
    fn parse_decimal_overflow(&mut self, positive: bool, significand: u64, exponent: i32) -> Result<f64, &str> {
        // The next multiply/add would overflow, so just ignore all further
        // digits.
        while let Ok(b'0'..=b'9') = self.peek_or_null() {
            self.eat_char();
        }

        match self.peek_or_null().ok() {
            Some(b'e') | Some(b'E') => self.parse_exponent(positive, significand, exponent),
            _ => self.f64_from_parts(positive, significand, exponent),
        }
    }
}

#[test]
fn test_parse_decimal_overflow_with_zeroes() {
    let mut parser = MockParser::new(b"000000");
    let result = parser.parse_decimal_overflow(true, 123, 2);
    assert_eq!(result, Ok(0.0));
}

#[test]
fn test_parse_decimal_overflow_with_no_digits() {
    let mut parser = MockParser::new(b"123.abc");
    let result = parser.parse_decimal_overflow(true, 123, 2);
    assert_eq!(result, Ok(0.0));
}

#[test]
fn test_parse_decimal_overflow_with_exponent() {
    let mut parser = MockParser::new(b"123.456e2");
    let result = parser.parse_decimal_overflow(true, 123456, 2);
    assert_eq!(result, Ok(0.0));
}

#[test]
fn test_parse_decimal_overflow_empty_input() {
    let mut parser = MockParser::new(b"");
    let result = parser.parse_decimal_overflow(true, 123, 2);
    assert_eq!(result, Ok(0.0));
}

