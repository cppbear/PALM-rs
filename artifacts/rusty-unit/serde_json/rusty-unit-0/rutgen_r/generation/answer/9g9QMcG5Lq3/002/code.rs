// Answer 0

fn parse_exponent_overflow_test() -> Result<()> {
    struct MockParser {
        data: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn peek_or_null(&self) -> Result<u8> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Ok(b'\0') // simulating end of input
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.data.len() {
                self.index += 1;
            }
        }

        fn error(&self, _code: ErrorCode) -> Error {
            // Mock implementation of an error
            Error::new("mock error")
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

    // Test 1: zero_significand is true
    let mut parser = MockParser::new(vec![]); // No digits
    let result = parser.parse_exponent_overflow(true, true, true);
    assert_eq!(result, Ok(0.0));

    // Test 2: Check behavior with digits present
    let mut parser = MockParser::new(vec![b'0', b'1', b'2']); // Some digits
    let result = parser.parse_exponent_overflow(true, true, true);
    assert_eq!(result, Ok(0.0));

    // Test 3: Check with zero_significand false and positive exponent to trigger error
    let mut parser = MockParser::new(vec![b'0']); // Some digits
    let result = parser.parse_exponent_overflow(true, false, true);
    assert_eq!(result.is_err(), true);

    // Test 4: Validate behavior with no input
    let mut parser = MockParser::new(vec![]); // No digits
    let result = parser.parse_exponent_overflow(false, true, false);
    assert_eq!(result, Ok(-0.0));

    Ok(())
}

#[test]
fn run_parse_exponent_overflow_tests() {
    parse_exponent_overflow_test().unwrap();
}

