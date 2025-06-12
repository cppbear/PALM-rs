// Answer 0

#[test]
#[should_panic]
fn test_parse_number_peek_or_null_err() {
    struct MockParser {
        should_err: bool,
    }

    impl MockParser {
        fn peek_or_null(&mut self) -> Result<u8, &'static str> {
            if self.should_err {
                Err("peek failed")
            } else {
                Ok(b'1')
            }
        }

        fn parse_decimal(&mut self, _positive: bool, _significand: u64, _decimals: u32) -> Result<f64, &'static str> {
            Ok(1.0)
        }

        fn parse_exponent(&mut self, _positive: bool, _significand: u64, _decimals: u32) -> Result<f64, &'static str> {
            Ok(1.0)
        }
    }

    let mut parser = MockParser { should_err: true };
    let positive = true;
    let significand = 42;

    let result = parser.parse_number(positive, significand);
    assert!(result.is_err());
}

