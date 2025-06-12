// Answer 0

#[test]
fn test_parse_number_positive_significand() {
    struct TestParser {
        state: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn new(state: Vec<u8>) -> Self {
            Self { state, index: 0 }
        }

        fn peek_or_null(&mut self) -> Result<u8, &str> {
            if self.index < self.state.len() {
                Ok(self.state[self.index])
            } else {
                Err("out of bounds")
            }
        }

        fn parse_decimal(&self, _positive: bool, _significand: u64, _scale: u64) -> Result<f64, &str> {
            Ok(123.456)
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, _scale: u64) -> Result<f64, &str> {
            Ok(1.23456e3)
        }

        fn parse_number(&mut self, positive: bool, significand: u64) -> Result<ParserNumber, &str> {
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

    let mut parser = TestParser::new(vec![b'.']);
    let result = parser.parse_number(false, 0);
    assert_eq!(result, Ok(ParserNumber::F64(123.456)));
}

#[test]
fn test_parse_number_exponent() {
    struct TestParser {
        state: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn new(state: Vec<u8>) -> Self {
            Self { state, index: 0 }
        }

        fn peek_or_null(&mut self) -> Result<u8, &str> {
            if self.index < self.state.len() {
                Ok(self.state[self.index])
            } else {
                Err("out of bounds")
            }
        }

        fn parse_decimal(&self, _positive: bool, _significand: u64, _scale: u64) -> Result<f64, &str> {
            Ok(123.456)
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, _scale: u64) -> Result<f64, &str> {
            Ok(1.23456e3)
        }

        fn parse_number(&mut self, positive: bool, significand: u64) -> Result<ParserNumber, &str> {
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

    let mut parser = TestParser::new(vec![b'e']);
    let result = parser.parse_number(false, 0);
    assert_eq!(result, Ok(ParserNumber::F64(1.23456e3)));
}

#[test]
fn test_parse_number_negative_significand() {
    struct TestParser {
        state: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn new(state: Vec<u8>) -> Self {
            Self { state, index: 0 }
        }

        fn peek_or_null(&mut self) -> Result<u8, &str> {
            if self.index < self.state.len() {
                Ok(self.state[self.index])
            } else {
                Err("out of bounds")
            }
        }

        fn parse_decimal(&self, _positive: bool, _significand: u64, _scale: u64) -> Result<f64, &str> {
            Ok(123.456)
        }

        fn parse_exponent(&self, _positive: bool, _significand: u64, _scale: u64) -> Result<f64, &str> {
            Ok(1.23456e3)
        }

        fn parse_number(&mut self, positive: bool, significand: u64) -> Result<ParserNumber, &str> {
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

    let mut parser = TestParser::new(vec![b'x']); // Use a non-special character
    let result = parser.parse_number(false, u64::MAX);
    assert_eq!(result, Ok(ParserNumber::I64(-1))); // Assert the result for the edge case
}

