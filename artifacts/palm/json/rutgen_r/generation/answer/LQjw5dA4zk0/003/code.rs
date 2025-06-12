// Answer 0

#[test]
fn test_parse_number_positive_zero_case() {
    struct DummyParser {
        state: Vec<u8>,
        index: usize,
    }

    impl DummyParser {
        fn new(state: Vec<u8>) -> Self {
            DummyParser { state, index: 0 }
        }
        
        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.state.len() {
                Ok(self.state[self.index])
            } else {
                Ok(0) // Simulate null termination
            }
        }
        
        fn parse_decimal(&mut self, _positive: bool, _significand: u64, _scale: u64) -> Result<f64, ()> {
            Ok(0.0) // Simulate a decimal parsing result
        }
        
        fn parse_exponent(&mut self, _positive: bool, _significand: u64, _scale: u64) -> Result<f64, ()> {
            Ok(0.0) // Simulate an exponent parsing result
        }
    }

    let mut parser = DummyParser::new(vec![b'0']);
    let result = parser.parse_number(false, 0);

    assert_eq!(result, Ok(ParserNumber::F64(0.0)));
}

#[test]
fn test_parse_number_negative_case() {
    struct DummyParser {
        state: Vec<u8>,
        index: usize,
    }

    impl DummyParser {
        fn new(state: Vec<u8>) -> Self {
            DummyParser { state, index: 0 }
        }
        
        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.state.len() {
                Ok(self.state[self.index])
            } else {
                Ok(0) // Simulate null termination
            }
        }
        
        fn parse_decimal(&mut self, _positive: bool, _significand: u64, _scale: u64) -> Result<f64, ()> {
            Ok(0.0) // Simulate a decimal parsing result
        }
        
        fn parse_exponent(&mut self, _positive: bool, _significand: u64, _scale: u64) -> Result<f64, ()> {
            Ok(0.0) // Simulate an exponent parsing result
        }
    }

    let mut parser = DummyParser::new(vec![b'0']);
    let result = parser.parse_number(false, 1);

    assert_eq!(result, Ok(ParserNumber::I64(-1)));
}

