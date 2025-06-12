// Answer 0

#[derive(Default)]
struct Parser {
    index: usize,
    chars: Vec<u8>,
}

impl Parser {
    fn eat_char(&mut self) {
        self.index += 1;
    }

    fn peek(&self) -> Result<Option<u8>, String> {
        if self.index < self.chars.len() {
            Ok(Some(self.chars[self.index]))
        } else {
            Ok(None)
        }
    }

    fn peek_or_null(&self) -> Result<u8, String> {
        if self.index < self.chars.len() {
            Ok(self.chars[self.index])
        } else {
            Err("End of input".to_string())
        }
    }

    fn parse_decimal_overflow(
        &self,
        _positive: bool,
        _significand: u64,
        _exponent: i32,
    ) -> Result<f64, String> {
        Err("Overflow".to_string())
    }

    fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, String> {
        Ok(0.0) // Mock implementation for testing
    }

    fn parse_exponent(&self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64, String> {
        Ok(0.0) // Mock implementation for testing
    }

    fn parse_decimal(
        &mut self,
        positive: bool,
        mut significand: u64,
        exponent_before_decimal_point: i32,
    ) -> Result<f64, String> {
        self.eat_char();
        
        let mut exponent_after_decimal_point = 0;
        while let c @ b'0'..=b'9' = self.peek_or_null().unwrap() {
            let digit = (c - b'0') as u64;

            if significand.checked_mul(10).map_or(true, |v| v + digit > u64::MAX) {
                let exponent = exponent_before_decimal_point + exponent_after_decimal_point;
                return self.parse_decimal_overflow(positive, significand, exponent);
            }

            self.eat_char();
            significand = significand * 10 + digit;
            exponent_after_decimal_point -= 1;
        }

        if exponent_after_decimal_point == 0 {
            if self.peek()?.is_some() {
                return Err("Invalid number".to_string());
            } else {
                return Err("EOF while parsing value".to_string());
            }
        }

        let exponent = exponent_before_decimal_point + exponent_after_decimal_point;
        match self.peek_or_null()? {
            b'e' | b'E' => self.parse_exponent(positive, significand, exponent),
            _ => self.f64_from_parts(positive, significand, exponent),
        }
    }
}

#[test]
fn test_parse_decimal_valid() {
    let mut parser = Parser { 
        chars: b"01.234".to_vec(),
        ..Default::default()
    };
    let result = parser.parse_decimal(true, 1, 0);
    assert_eq!(result, Ok(0.0));
}

#[test]
fn test_parse_decimal_invalid_no_digits_after_decimal() {
    let mut parser = Parser { 
        chars: b"123.".to_vec(),
        ..Default::default()
    };
    let result = parser.parse_decimal(true, 123, 0);
    assert_eq!(result, Err("Invalid number".to_string()));
}

#[test]
fn test_parse_decimal_invalid_eof_after_decimal() {
    let mut parser = Parser { 
        chars: b"123.".to_vec(),
        ..Default::default()
    };
    parser.eat_char(); // move past '1'
    parser.eat_char(); // move past '2'
    parser.eat_char(); // move past '3'
    parser.eat_char(); // move past '.'
    let result = parser.parse_decimal(true, 123, 0);
    assert_eq!(result, Err("EOF while parsing value".to_string()));
}

#[test]
fn test_parse_decimal_overflow() {
    let mut parser = Parser { 
        chars: b"9999999999.9999999999".to_vec(),
        ..Default::default()
    };
    let result = parser.parse_decimal(true, u64::MAX, 0);
    assert_eq!(result, Err("Overflow".to_string()));
}

