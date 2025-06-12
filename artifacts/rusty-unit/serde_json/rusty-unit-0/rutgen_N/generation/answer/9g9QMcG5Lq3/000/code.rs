// Answer 0

#[derive(Default)]
struct Parser {
    // Add additional fields as necessary for the parser's context.
}

impl Parser {
    fn peek_or_null(&mut self) -> Result<u8> {
        // Return a mocked value for this test's context
        Ok(b'0') // Placeholder return value
    }

    fn eat_char(&mut self) {
        // Placeholder for the eat_char behavior
    }

    fn error(&self, code: ErrorCode) -> Error {
        Error { code }
    }

    fn parse_exponent_overflow(
        &mut self,
        positive: bool,
        zero_significand: bool,
        positive_exp: bool,
    ) -> Result<f64> {
        if !zero_significand && positive_exp {
            return Err(self.error(ErrorCode::NumberOutOfRange));
        }

        while let b'0'..=b'9' = self.peek_or_null()? {
            self.eat_char();
        }
        Ok(if positive { 0.0 } else { -0.0 })
    }
}

#[derive(Debug)]
struct Error {
    code: ErrorCode,
}

#[derive(Debug)]
enum ErrorCode {
    NumberOutOfRange,
}

type Result<T> = std::result::Result<T, Error>;

#[test]
fn test_parse_exponent_overflow_positive_zero_significand() {
    let mut parser = Parser::default();
    let result = parser.parse_exponent_overflow(true, true, false);
    assert_eq!(result, Ok(0.0));
}

#[test]
fn test_parse_exponent_overflow_negative_zero_significand() {
    let mut parser = Parser::default();
    let result = parser.parse_exponent_overflow(false, true, false);
    assert_eq!(result, Ok(-0.0));
}

#[test]
fn test_parse_exponent_overflow_positive_non_zero_significand() {
    let mut parser = Parser::default();
    let result = parser.parse_exponent_overflow(true, false, true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, ErrorCode::NumberOutOfRange);
}

#[test]
fn test_parse_exponent_overflow_negative_non_zero_significand() {
    let mut parser = Parser::default();
    let result = parser.parse_exponent_overflow(false, false, true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, ErrorCode::NumberOutOfRange);
}

