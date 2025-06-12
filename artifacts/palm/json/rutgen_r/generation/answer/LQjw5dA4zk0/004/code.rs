// Answer 0

#[derive(Debug)]
enum ParserNumber {
    U64(u64),
    I64(i64),
    F64(f64),
}

struct MockParser {
    is_peek_or_null_ok: bool,
    peek_value: Option<u8>,
    parse_decimal_result: Result<f64, ()>,
    parse_exponent_result: Result<f64, ()>,
}

impl MockParser {
    fn peek_or_null(&mut self) -> Result<u8, ()> {
        if self.is_peek_or_null_ok {
            Ok(self.peek_value.unwrap_or(0))
        } else {
            Err(())
        }
    }

    fn parse_decimal(&mut self, _positive: bool, _significand: u64, _exp: u32) -> Result<f64, ()> {
        self.parse_decimal_result
    }

    fn parse_exponent(&mut self, _positive: bool, _significand: u64, _exp: u32) -> Result<f64, ()> {
        self.parse_exponent_result
    }

    fn parse_number(&mut self, positive: bool, significand: u64) -> Result<ParserNumber, ()> {
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
fn test_parse_number_with_decimal() {
    let mut parser = MockParser {
        is_peek_or_null_ok: true,
        peek_value: Some(b'.'),
        parse_decimal_result: Ok(12.34),
        parse_exponent_result: Ok(0.0),
    };
    let result = parser.parse_number(false, 0);
    assert!(result.is_ok());
    if let Ok(ParserNumber::F64(value)) = result {
        assert_eq!(value, -12.34);
    }
}

#[test]
fn test_parse_number_with_exponent() {
    let mut parser = MockParser {
        is_peek_or_null_ok: true,
        peek_value: Some(b'e'),
        parse_decimal_result: Ok(0.0),
        parse_exponent_result: Ok(34.0),
    };
    let result = parser.parse_number(false, 0);
    assert!(result.is_ok());
    if let Ok(ParserNumber::F64(value)) = result {
        assert_eq!(value, -34.0);
    }
}

#[test]
fn test_parse_number_positive() {
    let mut parser = MockParser {
        is_peek_or_null_ok: true,
        peek_value: Some(b'x'), // Not matching '.', 'e', or 'E'
        parse_decimal_result: Ok(0.0),
        parse_exponent_result: Ok(0.0),
    };
    let result = parser.parse_number(true, 42);
    assert!(result.is_ok());
    if let Ok(ParserNumber::U64(value)) = result {
        assert_eq!(value, 42);
    }
}

#[test]
fn test_parse_number_negative_underflow() {
    let mut parser = MockParser {
        is_peek_or_null_ok: true,
        peek_value: Some(b'x'), // Not matching '.', 'e', or 'E'
        parse_decimal_result: Ok(0.0),
        parse_exponent_result: Ok(0.0),
    };
    let result = parser.parse_number(false, u64::MAX); // causes underflow
    assert!(result.is_ok());
    if let Ok(ParserNumber::F64(value)) = result {
        assert_eq!(value, -u64::MAX as f64);
    }
}

