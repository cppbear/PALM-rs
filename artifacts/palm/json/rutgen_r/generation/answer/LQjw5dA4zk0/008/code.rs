// Answer 0

#[test]
fn test_parse_number_positive_u64() {
    struct MockParser {
        peek_value: u8,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, ()> {
            Ok(self.peek_value)
        }
        
        fn parse_decimal(&mut self, positive: bool, significand: u64, _exp: u32) -> Result<f64, ()> {
            Ok(significand as f64)
        }

        fn parse_exponent(&mut self, positive: bool, significand: u64, _exp: u32) -> Result<f64, ()> {
            Ok(significand as f64 * 10f64.powi(1)) // sample exponent
        }
    }

    let mut parser = MockParser { peek_value: b'0' }; // In this case, the peek_value is not affecting the output.
    let result = parser.parse_number(true, 12345);
    assert_eq!(result, Ok(ParserNumber::U64(12345)));
}

#[test]
fn test_parse_number_negative_i64() {
    struct MockParser {
        peek_value: u8,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, ()> {
            Ok(self.peek_value)
        }

        fn parse_decimal(&mut self, positive: bool, significand: u64, _exp: u32) -> Result<f64, ()> {
            Ok(significand as f64)
        }

        fn parse_exponent(&mut self, positive: bool, significand: u64, _exp: u32) -> Result<f64, ()> {
            Ok(significand as f64 * 10f64.powi(1)) // sample exponent
        }
    }

    let mut parser = MockParser { peek_value: b'0' };
    let result = parser.parse_number(false, 12345);
    assert_eq!(result, Ok(ParserNumber::I64(-12345)));
}

#[test]
fn test_parse_number_negative_float() {
    struct MockParser {
        peek_value: u8,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, ()> {
            Ok(self.peek_value)
        }

        fn parse_decimal(&mut self, positive: bool, significand: u64, _exp: u32) -> Result<f64, ()> {
            Ok(significand as f64)
        }

        fn parse_exponent(&mut self, positive: bool, significand: u64, _exp: u32) -> Result<f64, ()> {
            Ok(significand as f64 * 10f64.powi(1)) // sample exponent
        }
    }

    let mut parser = MockParser { peek_value: b'0' };
    let result = parser.parse_number(false, 0);
    assert_eq!(result, Ok(ParserNumber::F64(0.0)));
}

#[test]
fn test_parse_number_float_with_exponent() {
    struct MockParser {
        peek_value: u8,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, ()> {
            Ok(self.peek_value)
        }

        fn parse_decimal(&mut self, positive: bool, significand: u64, _exp: u32) -> Result<f64, ()> {
            Ok(significand as f64)
        }

        fn parse_exponent(&mut self, positive: bool, significand: u64, _exp: u32) -> Result<f64, ()> {
            Ok(significand as f64 * 10f64.powi(1)) // sample exponent
        }
    }

    let mut parser = MockParser { peek_value: b'e' };
    let result = parser.parse_number(true, 12345);
    assert_eq!(result, Ok(ParserNumber::F64(123450.0)));
}

#[test]
fn test_parse_number_decimal() {
    struct MockParser {
        peek_value: u8,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, ()> {
            Ok(self.peek_value)
        }

        fn parse_decimal(&mut self, positive: bool, significand: u64, _exp: u32) -> Result<f64, ()> {
            Ok(significand as f64 + 0.1) // example decimal addition
        }

        fn parse_exponent(&mut self, positive: bool, significand: u64, _exp: u32) -> Result<f64, ()> {
            Ok(significand as f64 * 10f64.powi(1)) // sample exponent
        }
    }

    let mut parser = MockParser { peek_value: b'.' };
    let result = parser.parse_number(true, 12345);
    assert_eq!(result, Ok(ParserNumber::F64(12345.1)));
}

