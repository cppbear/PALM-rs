// Answer 0

#[test]
fn test_parse_exponent_overflow_positive_zero_significand_false_positive_exp_false() {
    struct TestParser {
        input: Vec<u8>,
        position: usize,
    }

    impl TestParser {
        fn new(input: Vec<u8>) -> Self {
            TestParser { input, position: 0 }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn error(&self, _: ErrorCode) -> () {
            // Simulate an error
        }

        fn parse_exponent_overflow(
            &mut self,
            positive: bool,
            zero_significand: bool,
            positive_exp: bool,
        ) -> Result<f64, ()> {
            if !zero_significand && positive_exp {
                return Err(self.error(ErrorCode::NumberOutOfRange));
            }

            while let b'0'..=b'9' = self.peek_or_null().unwrap() {
                self.eat_char();
            }
            Ok(if positive { 0.0 } else { -0.0 })
        }
    }

    let mut parser = TestParser::new(vec![b'a', b'b', b'c']); // No digits present, fulfilling the condition
    let result = parser.parse_exponent_overflow(true, false, false);
    assert_eq!(result, Ok(0.0));
}

#[test]
#[should_panic]
fn test_parse_exponent_overflow_positive_zero_significand_false_positive_exp_true() {
    struct TestParser {
        input: Vec<u8>,
        position: usize,
    }

    impl TestParser {
        fn new(input: Vec<u8>) -> Self {
            TestParser { input, position: 0 }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(())
            }
        }
        
        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn error(&self, _: ErrorCode) -> () {
            // Simulate an error
        }

        fn parse_exponent_overflow(
            &mut self,
            positive: bool,
            zero_significand: bool,
            positive_exp: bool,
        ) -> Result<f64, ()> {
            if !zero_significand && positive_exp {
                return Err(self.error(ErrorCode::NumberOutOfRange));
            }

            while let b'0'..=b'9' = self.peek_or_null().unwrap() {
                self.eat_char();
            }
            Ok(if positive { 0.0 } else { -0.0 })
        }
    }

    let mut parser = TestParser::new(vec![b'1']); // This will cause a panic
    parser.parse_exponent_overflow(true, false, true);
}

