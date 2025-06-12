// Answer 0

#[test]
fn test_parse_number_positive_significand() {
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
        
        fn parse_decimal(&mut self, _: bool, significand: u64, _: u32) -> Result<f64, ()> {
            Ok(significand as f64) // simplistic conversion for testing
        }
        
        fn parse_exponent(&mut self, _: bool, significand: u64, _: u32) -> Result<f64, ()> {
            Ok(significand as f64 * 10f64) // simplistic conversion for testing
        }
        
        fn advance(&mut self) {
            self.position += 1;
        }
    }
    
    let mut parser = TestParser::new(vec![b'1']);
    let result = parser.parse_number(true, 123);
    
    assert_eq!(result, Ok(ParserNumber::U64(123)));
}

#[test]
fn test_parse_number_decimal() {
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

        fn parse_decimal(&mut self, _: bool, _: u64, _: u32) -> Result<f64, ()> {
            Ok(3.14) // example decimal for test
        }

        fn advance(&mut self) {
            self.position += 1;
        }
    }

    let mut parser = TestParser::new(vec![b'.']);
    parser.advance(); // Move past the dot
    let result = parser.parse_number(true, 0);
    
    assert_eq!(result, Ok(ParserNumber::F64(3.14)));
}

#[test]
fn test_parse_number_exponent() {
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

        fn parse_exponent(&mut self, _: bool, significand: u64, _: u32) -> Result<f64, ()> {
            Ok(significand as f64 * 1000.0) // example for test
        }

        fn advance(&mut self) {
            self.position += 1;
        }
    }

    let mut parser = TestParser::new(vec![b'e']);
    parser.advance(); // Move past the exponent
    let result = parser.parse_number(true, 2);
    
    assert_eq!(result, Ok(ParserNumber::F64(2000.0)));
}

