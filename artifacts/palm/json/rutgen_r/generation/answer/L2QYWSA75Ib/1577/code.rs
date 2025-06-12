// Answer 0

#[test]
fn test_ignore_value_empty_object() {
    struct MockParser {
        scratch: Vec<u8>,
    }

    impl MockParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            // Simulate end of input
            Ok(None)
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }

        fn read(&self) -> &Self {
            self
        }

        fn ignore_str(&self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error::new("error")
        }
    }

    let mut parser = MockParser {
        scratch: Vec::new(),
    };

    let result = parser.ignore_value();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_value_empty_array() {
    struct MockParser {
        scratch: Vec<u8>,
    }

    impl MockParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            // Simulate end of input
            Ok(None)
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }

        fn read(&self) -> &Self {
            self
        }

        fn ignore_str(&self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error::new("error")
        }
    }

    let mut parser = MockParser {
        scratch: Vec::new(),
    };

    let result = parser.ignore_value();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_ignore_value_invalid_character() {
    struct MockParser {
        scratch: Vec<u8>,
    }

    impl MockParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            // Simulate reading an invalid character
            Ok(Some(b'x'))
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }

        fn read(&self) -> &Self {
            self
        }

        fn ignore_str(&self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error::new("error")
        }
    }

    let mut parser = MockParser {
        scratch: Vec::new(),
    };

    let _ = parser.ignore_value();
}

#[test]
fn test_ignore_value_null() {
    struct MockParser {
        scratch: Vec<u8>,
    }

    impl MockParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            // Simulate reading 'n'
            static mut CALL_COUNT: i32 = 0;
            unsafe {
                CALL_COUNT += 1;
                if CALL_COUNT == 1 {
                    return Ok(Some(b'n'));
                }
                return Ok(Some(b' '));
            }
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }

        fn read(&self) -> &Self {
            self
        }

        fn ignore_str(&self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error::new("error")
        }
    }

    let mut parser = MockParser {
        scratch: Vec::new(),
    };

    let result = parser.ignore_value();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_value_true() {
    struct MockParser {
        scratch: Vec<u8>,
    }

    impl MockParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            // Simulate reading 't'
            static mut CALL_COUNT: i32 = 0;
            unsafe {
                CALL_COUNT += 1;
                if CALL_COUNT == 1 {
                    return Ok(Some(b't'));
                }
                return Ok(Some(b' '));
            }
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }

        fn read(&self) -> &Self {
            self
        }

        fn ignore_str(&self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error::new("error")
        }
    }

    let mut parser = MockParser {
        scratch: Vec::new(),
    };

    let result = parser.ignore_value();
    assert!(result.is_ok());
}

