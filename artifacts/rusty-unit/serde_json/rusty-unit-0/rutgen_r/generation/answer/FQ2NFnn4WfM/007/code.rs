// Answer 0

fn parse_long_integer_tests() {
    struct MockParser {
        data: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn parse_decimal(&self, _positive: bool, _significand: u64, _exponent: u32) -> Result<f64, ()> {
            // Mock implementation
            Ok(0.0)
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, _exponent: u32) -> Result<f64, ()> {
            // Mock implementation
            Ok(0.0)
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exponent: u32) -> Result<f64, ()> {
            // Mock implementation
            Ok(0.0)
        }
    }

    impl MockParser {
        fn parse_long_integer(&mut self, positive: bool, significand: u64) -> Result<f64, ()> {
            let mut exponent = 0;
            loop {
                match self.peek_or_null() {
                    Ok(val) if (b'0'..=b'9').contains(&val) => {
                        self.eat_char();
                        exponent += 1;
                    }
                    Ok(b'.') => {
                        return self.parse_decimal(positive, significand, exponent);
                    }
                    Ok(b'e') | Ok(b'E') => {
                        return self.parse_exponent(positive, significand, exponent);
                    }
                    _ => {
                        return self.f64_from_parts(positive, significand, exponent);
                    }
                }
            }
        }
    }

    #[test]
    fn test_positive_significand_no_digits() {
        let mut parser = MockParser::new(vec![]);
        let result = parser.parse_long_integer(true, 1);
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn test_negative_significand_no_digits() {
        let mut parser = MockParser::new(vec![]);
        let result = parser.parse_long_integer(false, 1);
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn test_exponent_character() {
        let mut parser = MockParser::new(vec![b'e', b'1']);
        let result = parser.parse_long_integer(true, 1);
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn test_decimal_character() {
        let mut parser = MockParser::new(vec![b'.']);
        let result = parser.parse_long_integer(true, 1);
        assert_eq!(result, Ok(0.0));
    }
}

