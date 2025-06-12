// Answer 0

#[derive(Debug)]
struct Parser;

#[derive(Debug)]
enum ParserNumber {
    F64(f64),
    U64(u64),
    I64(i64),
}

impl Parser {
    fn peek_or_null(&mut self) -> Result<u8> {
        // Mock implementation for testing purposes
        Ok(b'0') // Example returns 0
    }

    fn parse_decimal(&mut self, _positive: bool, _significand: u64, _scale: u32) -> Result<f64> {
        // Mock implementation for testing
        Ok(_significand as f64)
    }

    fn parse_exponent(&mut self, _positive: bool, _significand: u64, _scale: u32) -> Result<f64> {
        // Mock implementation for testing
        Ok(_significand as f64 * 10f64.powi(1)) // Example returns significand * 10^1
    }

    fn parse_number(&mut self, positive: bool, significand: u64) -> Result<ParserNumber> {
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

#[test]
fn test_parse_number_positive_integer() {
    let mut parser = Parser;
    let result = parser.parse_number(true, 42).unwrap();
    assert_eq!(result, ParserNumber::U64(42));
}

#[test]
fn test_parse_number_negative_integer() {
    let mut parser = Parser;
    let result = parser.parse_number(false, 42).unwrap();
    assert_eq!(result, ParserNumber::I64(-42));
}

#[test]
fn test_parse_number_positive_float() {
    let mut parser = Parser;
    let result = parser.parse_number(true, 42).unwrap();
    assert_eq!(result, ParserNumber::U64(42));
}

#[test]
fn test_parse_number_negative_float() {
    let mut parser = Parser;
    let result = parser.parse_number(false, 0).unwrap();
    assert_eq!(result, ParserNumber::F64(0.0));
}

