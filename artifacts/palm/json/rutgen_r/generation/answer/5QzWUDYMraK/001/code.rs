// Answer 0

#[test]
fn test_parse_any_number_positive_case() {
    struct TestParser {
        input: i32,
    }
    
    impl TestParser {
        fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber, String> {
            if positive && self.input < 0 {
                return Err("Input must be positive".to_string());
            }
            Ok(ParserNumber(self.input))
        }
        
        fn parse_any_number(&mut self, positive: bool) -> Result<ParserNumber, String> {
            self.parse_integer(positive)
        }
    }

    let mut parser = TestParser { input: 42 };
    let result = parser.parse_any_number(true);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ParserNumber(42));
}

#[test]
fn test_parse_any_number_negative_case() {
    struct TestParser {
        input: i32,
    }
    
    impl TestParser {
        fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber, String> {
            if positive && self.input < 0 {
                return Err("Input must be positive".to_string());
            }
            Ok(ParserNumber(self.input))
        }
        
        fn parse_any_number(&mut self, positive: bool) -> Result<ParserNumber, String> {
            self.parse_integer(positive)
        }
    }

    let mut parser = TestParser { input: -5 };
    let result = parser.parse_any_number(true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Input must be positive");
}

#[test]
fn test_parse_any_number_zero_case() {
    struct TestParser {
        input: i32,
    }

    impl TestParser {
        fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber, String> {
            if positive && self.input < 0 {
                return Err("Input must be positive".to_string());
            }
            Ok(ParserNumber(self.input))
        }
        
        fn parse_any_number(&mut self, positive: bool) -> Result<ParserNumber, String> {
            self.parse_integer(positive)
        }
    }

    let mut parser = TestParser { input: 0 };
    let result = parser.parse_any_number(true);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ParserNumber(0));
}

#[derive(Debug, PartialEq)]
struct ParserNumber(i32);

