// Answer 0

#[derive(Debug)]
struct TestParser {
    state: Vec<u8>,
}

impl TestParser {
    fn peek_or_null(&mut self) -> Result<u8, &'static str> {
        if self.state.is_empty() {
            Ok(0) // represents null
        } else {
            Ok(self.state[0])
        }
    }

    fn parse_decimal(&self, positive: bool, significand: u64, _: u64) -> Result<f64, &'static str> {
        if positive {
            Ok(significand as f64)
        } else {
            Ok(-(significand as f64))
        }
    }

    fn parse_exponent(&self, positive: bool, significand: u64, _: u64) -> Result<f64, &'static str> {
        if positive {
            Ok(significand as f64)
        } else {
            Ok(-(significand as f64))
        }
    }

    fn parse_number(&mut self, positive: bool, significand: u64) -> Result<ParserNumber, &'static str> {
        Ok(match self.peek_or_null()? {
            b'.' => ParserNumber::F64(self.parse_decimal(positive, significand, 0)?),
            b'e' | b'E' => ParserNumber::F64(self.parse_exponent(positive, significand, 0)?),
            _ => {
                if positive {
                    ParserNumber::U64(significand)
                } else {
                    let neg = (significand as i64).wrapping_neg();
                    if neg >= 0 {
                        ParserNumber::F64(-(significand as f64))
                    } else {
                        ParserNumber::I64(neg)
                    }
                }
            }
        })
    }
}

#[derive(Debug, PartialEq)]
enum ParserNumber {
    F64(f64),
    U64(u64),
    I64(i64),
}

#[test]
fn test_parse_number_positive() {
    let mut parser = TestParser { state: vec![b'1'] };
    let result = parser.parse_number(true, 42);
    assert_eq!(result, Ok(ParserNumber::U64(42)));
}

#[test]
fn test_parse_number_negative_float_conversion() {
    let mut parser = TestParser { state: vec![b'1'] };
    let result = parser.parse_number(false, 0);
    assert_eq!(result, Ok(ParserNumber::F64(-0.0)));
}

#[test]
fn test_parse_number_negative_i64_conversion() {
    let mut parser = TestParser { state: vec![b'1'] };
    let result = parser.parse_number(false, 43);
    assert_eq!(result, Ok(ParserNumber::I64(-43)));
}

#[test]
fn test_parse_number_with_decimal() {
    let mut parser = TestParser { state: vec![b'.'] };
    let result = parser.parse_number(false, 42);
    assert_eq!(result, Ok(ParserNumber::F64(-42.0)));
}

#[test]
fn test_parse_number_with_exponent() {
    let mut parser = TestParser { state: vec![b'e'] };
    let result = parser.parse_number(false, 42);
    assert_eq!(result, Ok(ParserNumber::F64(-42.0)));
}

#[test]
fn test_parse_number_zero() {
    let mut parser = TestParser { state: vec![b'0'] };
    let result = parser.parse_number(false, 0);
    assert_eq!(result, Ok(ParserNumber::F64(-0.0)));
}

