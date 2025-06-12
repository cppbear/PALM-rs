// Answer 0

#[test]
fn test_ignore_value_eof_while_parsing_value() {
    struct MockParser {
        scratch: Vec<u8>,
    }

    impl MockParser {
        fn new() -> Self {
            Self { scratch: Vec::new() }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, ()> {
            Ok(None) // This simulates an EOF
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), ()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<(), ()> {
            Ok(())
        }
        
        fn read(&mut self) -> &Self {
            self
        }
        
        fn ignore_str(&mut self) -> Result<(), ()> {
            Ok(())
        }
        
        fn peek_error(&self, _: ErrorCode) -> () {
            ()
        }
    }

    let mut parser = MockParser::new();
    let result = parser.ignore_value();
    assert!(result.is_err());
}

#[test]
fn test_ignore_value_expected_some_value() {
    struct MockParser {
        scratch: Vec<u8>,
        whitespace: bool,
        current: u8,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                scratch: vec![b'n', b'u', b'l', b'l'],
                whitespace: true,
                current: b'n',
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, ()> {
            if self.whitespace {
                self.whitespace = false;
                Ok(Some(b'n')) // Simulating valid whitespace
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.scratch.remove(0); // Consume next char
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), ()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<(), ()> {
            Ok(())
        }

        fn read(&mut self) -> &Self {
            self
        }

        fn ignore_str(&mut self) -> Result<(), ()> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            ()
        }
    }

    let mut parser = MockParser::new();
    let result = parser.ignore_value();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_value_expected_object_comma_or_end() {
    struct MockParser {
        scratch: Vec<u8>,
    }

    impl MockParser {
        fn new() -> Self {
            Self { scratch: vec![b'{', b':', b'}'] }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, ()> {
            Ok(Some(b',')) // Simulate that we see a comma when we expect it
        }

        fn eat_char(&mut self) {
            self.scratch.remove(0); // Consume next char
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), ()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<(), ()> {
            Ok(())
        }

        fn read(&mut self) -> &Self {
            self
        }

        fn ignore_str(&mut self) -> Result<(), ()> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            ()
        }
    }

    let mut parser = MockParser::new();
    let result = parser.ignore_value();
    assert!(result.is_err());
} 

#[test]
#[should_panic]
fn test_ignore_value_key_must_be_a_string() {
    struct MockParser {
        scratch: Vec<u8>,
    }

    impl MockParser {
        fn new() -> Self {
            Self { scratch: vec![b'{', b'1'] } // Invalid key to trigger panic
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, ()> {
            Ok(Some(b'"')) // Simulate finding a valid character
        }

        fn eat_char(&mut self) {
            self.scratch.remove(0); // Consume next char
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), ()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<(), ()> {
            Ok(())
        }

        fn read(&mut self) -> &Self {
            self
        }

        fn ignore_str(&mut self) -> Result<(), ()> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> () {
            ()
        }
    }

    let mut parser = MockParser::new();
    parser.ignore_value();
}

