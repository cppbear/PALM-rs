// Answer 0

#[test]
fn test_parse_integer_valid_single_digit() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn parse_number(&self, _positive: bool, significand: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(significand))
        }

        fn parse_long_integer(&self, _positive: bool, significand: u64) -> Result<u64, ()> {
            Ok(significand)
        }

        fn error(&self, _: ErrorCode) -> () {
            // Error handling stub
        }
        
        fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber, ()> {
            // implementation as provided
        }
    }

    let mut parser = MockParser { input: vec![b'5'], index: 0 };
    let result = parser.parse_integer(true);
    assert_eq!(result, Ok(ParserNumber::U64(5)));
}

#[test]
fn test_parse_integer_valid_multiple_digits() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            // Similar implementation as above
        }

        fn peek_or_null(&self) -> Result<Option<u8>, ()> {
            // Similar implementation as above
        }

        fn parse_number(&self, _positive: bool, significand: u64) -> Result<ParserNumber, ()> {
            Ok(ParserNumber::U64(significand))
        }

        fn parse_long_integer(&self, _positive: bool, significand: u64) -> Result<u64, ()> {
            Ok(significand)
        }

        fn error(&self, _: ErrorCode) -> () {
            // Error handling stub
        }
        
        fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber, ()> {
            // implementation as provided
        }
    }

    let mut parser = MockParser { input: vec![b'1', b'2', b'3'], index: 0 };
    let result = parser.parse_integer(true);
    assert_eq!(result, Ok(ParserNumber::U64(123)));
}

#[test]
fn test_parse_integer_invalid_leading_zero() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            // Similar implementation as above
        }

        fn peek_or_null(&self) -> Result<Option<u8>, ()> {
            // Similar implementation as above
        }

        fn parse_number(&self, _: bool, _: u64) -> Result<ParserNumber, ()> {
            Err(())
        }

        fn error(&self, _: ErrorCode) -> () {
            // Error handling stub
        }
        
        fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber, ()> {
            // implementation as provided
        }
    }

    let mut parser = MockParser { input: vec![b'0', b'1'], index: 0 };
    let result = parser.parse_integer(true);
    assert_eq!(result, Err(()));
} 

#[test]
fn test_parse_integer_invalid_non_digit() {
    struct MockParser {
        input: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            // Similar implementation as above
        }

        fn peek_or_null(&self) -> Result<Option<u8>, ()> {
            // Similar implementation as above
        }

        fn error(&self, _: ErrorCode) -> () {
            // Error handling stub
        }
        
        fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber, ()> {
            // implementation as provided
        }
    }

    let mut parser = MockParser { input: vec![b'a'], index: 0 };
    let result = parser.parse_integer(true);
    assert_eq!(result, Err(()));
}

