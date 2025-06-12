// Answer 0

fn test_ignore_value_ok() -> Result<()> {
    struct MockParser {
        scratch: Vec<u8>,
        input: Vec<u8>,
    }

    impl MockParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if !self.input.is_empty() {
                Ok(Some(self.input.remove(0)))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            // Simulating consumption of a character
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> String {
            "Error".to_string()
        }
    }

    let mut parser = MockParser {
        scratch: Vec::new(),
        input: b"true, false, null".to_vec(),
    };

    let result = parser.ignore_value();
    assert!(result.is_ok());
    Ok(())
}

fn test_ignore_value_eof_while_parsing_value() -> Result<()> {
    struct MockParser {
        scratch: Vec<u8>,
        input: Vec<u8>,
    }

    impl MockParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Err(ErrorCode::EofWhileParsingValue)
        }

        fn peek_error(&self, _: ErrorCode) -> String {
            "Error".to_string()
        }
    }

    let mut parser = MockParser {
        scratch: Vec::new(),
        input: vec![],
    };

    let result = parser.ignore_value();
    assert!(result.is_err());
    Ok(())
}

fn test_ignore_value_expected_some_value() -> Result<()> {
    struct MockParser {
        scratch: Vec<u8>,
        input: Vec<u8>,
    }

    impl MockParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'x')) // Simulating unexpected character
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> String {
            "Error".to_string()
        }
    }

    let mut parser = MockParser {
        scratch: Vec::new(),
        input: vec![],
    };

    let result = parser.ignore_value();
    assert!(result.is_err());
    Ok(())
}

fn test_ignore_value_key_must_be_a_string() -> Result<()> {
    struct MockParser {
        scratch: Vec<u8>,
        input: Vec<u8>,
    }

    impl MockParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{')) // Start an object
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> String {
            "Error".to_string()
        }

        fn read_ignore_str(&mut self) -> Result<()> {
            Err(ErrorCode::KeyMustBeAString) // Simulating an error
        }
    }

    let mut parser = MockParser {
        scratch: Vec::new(),
        input: vec![],
    };

    let result = parser.ignore_value();
    assert!(result.is_err());
    Ok(())
}

