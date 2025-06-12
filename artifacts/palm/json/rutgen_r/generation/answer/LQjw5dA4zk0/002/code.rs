// Answer 0

#[derive(Debug)]
enum ParserNumber {
    F64(f64),
    U64(u64),
    I64(i64),
}

struct Parser {
    data: Vec<u8>,
    position: usize,
}

impl Parser {
    fn peek_or_null(&mut self) -> Result<u8, ()> {
        if self.position < self.data.len() {
            Ok(self.data[self.position])
        } else {
            Err(())
        }
    }

    fn parse_decimal(&mut self, _positive: bool, _significand: u64, _scale: u64) -> Result<f64, ()> {
        Ok(_significand as f64)
    }

    fn parse_exponent(&mut self, _positive: bool, _significand: u64, _scale: u64) -> Result<f64, ()> {
        Ok(_significand as f64)
    }

    fn parse_number(&mut self, positive: bool, significand: u64) -> Result<ParserNumber, ()> {
        Ok(match self.peek_or_null().unwrap() {
            b'.' => ParserNumber::F64(self.parse_decimal(positive, significand, 0).unwrap()),
            b'e' | b'E' => ParserNumber::F64(self.parse_exponent(positive, significand, 0).unwrap()),
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
fn test_parse_number_positive_u64() {
    let mut parser = Parser { data: vec![b'1'], position: 0 };
    let result = parser.parse_number(true, 42);
    assert_eq!(result, Ok(ParserNumber::U64(42)));
}

#[test]
fn test_parse_number_decimal() {
    let mut parser = Parser { data: vec![b'.'], position: 0 };
    let result = parser.parse_number(true, 42);
    assert_eq!(result, Ok(ParserNumber::F64(42.0)));
}

#[test]
fn test_parse_number_exponent() {
    let mut parser = Parser { data: vec![b'e'], position: 0 };
    let result = parser.parse_number(true, 42);
    assert_eq!(result, Ok(ParserNumber::F64(42.0)));
}

#[test]
#[should_panic]
fn test_parse_number_out_of_bounds() {
    let mut parser = Parser { data: vec![], position: 0 };
    let _result = parser.parse_number(true, 42);
}

