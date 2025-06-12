// Answer 0

#[derive(Debug)]
struct ParserNumber {
    value: f64,
}

impl ParserNumber {
    fn F64(value: f64) -> Self {
        ParserNumber { value }
    }

    fn U64(value: u64) -> Self {
        ParserNumber { value: value as f64 }
    }

    fn I64(value: i64) -> Self {
        ParserNumber { value: value as f64 }
    }
}

struct Parser {
    data: Vec<u8>,
    index: usize,
}

impl Parser {
    fn new(data: Vec<u8>) -> Self {
        Parser { data, index: 0 }
    }

    fn peek_or_null(&mut self) -> Result<u8, ()> {
        if self.index < self.data.len() {
            Ok(self.data[self.index])
        } else {
            Ok(0)
        }
    }

    fn parse_decimal(&mut self, _positive: bool, significand: u64, _zero: u64) -> Result<f64, ()> {
        Ok(significand as f64)
    }

    fn parse_exponent(&mut self, _positive: bool, significand: u64, _zero: u64) -> Result<f64, ()> {
        Ok(significand as f64)
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
fn test_parse_number_positive_significand() {
    let mut parser = Parser::new(vec![b'0']);
    let result = parser.parse_number(true, 42).unwrap();
    assert_eq!(result.value, 42.0);
}

#[test]
fn test_parse_number_negative_significand() {
    let mut parser = Parser::new(vec![b'0']);
    let result = parser.parse_number(false, 42).unwrap();
    assert_eq!(result.value, -42.0);
}

#[test]
fn test_parse_number_decimal() {
    let mut parser = Parser::new(vec![b'.']);
    let result = parser.parse_number(true, 42).unwrap();
    assert_eq!(result.value, 42.0);
}

#[test]
fn test_parse_number_exponent() {
    let mut parser = Parser::new(vec![b'e']);
    let result = parser.parse_number(true, 42).unwrap();
    assert_eq!(result.value, 42.0);
}

#[test]
fn test_parse_number_edge_case() {
    let mut parser = Parser::new(vec![]);
    let result = parser.parse_number(true, 0).unwrap();
    assert_eq!(result.value, 0.0);
}

#[test]
fn test_parse_number_zero_significand_negative() {
    let mut parser = Parser::new(vec![b'0']);
    let result = parser.parse_number(false, 0).unwrap();
    assert_eq!(result.value, 0.0);
}

