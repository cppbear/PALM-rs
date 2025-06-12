// Answer 0

#[test]
fn test_parse_exponent_overflow_error() {
    struct Parser {
        pos: usize,
        input: Vec<u8>,
    }
    
    impl Parser {
        fn new(input: &str) -> Self {
            Self {
                pos: 0,
                input: input.bytes().collect(),
            }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(b'\0') // Return null byte if out of bounds
            }
        }

        fn eat_char(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn error(&self, _code: ErrorCode) -> Error {
            Error::new("Number out of range")
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

    enum ErrorCode {
        NumberOutOfRange,
    }

    struct Error {
        message: &'static str,
    }

    impl Error {
        fn new(message: &'static str) -> Self {
            Self { message }
        }
    }

    let mut parser = Parser::new("123456789012345678901234567890");
    let result = parser.parse_exponent_overflow(true, false, true);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().message, "Number out of range");
}

