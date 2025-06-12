// Answer 0

#[test]
fn test_ignore_value_valid_true() {
    struct TestParser {
        scratch: Vec<u8>,
        input: Vec<u8>,
    }

    impl TestParser {
        fn new(input: Vec<u8>) -> Self {
            Self { scratch: Vec::new(), input }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, Error> {
            if self.input.is_empty() {
                return Ok(None);
            }
            let byte = self.input.remove(0);
            Ok(Some(byte))
        }

        fn eat_char(&mut self) {
            // A no-op for this test as we remove in parse_whitespace
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<(), Error> {
            let ident_len = ident.len();
            if self.input.len() >= ident_len && &self.input[..ident_len] == ident {
                self.input.drain(..ident_len);
                Ok(())
            } else {
                Err(Error::ParseError)
            }
        }

        fn read_ignore_str(&mut self) -> Result<(), Error> {
            if self.input.get(0) == Some(&b'"') {
                self.input.remove(0);
                Ok(())
            } else {
                Err(Error::ParseError)
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::ParseError
        }
    }

    let mut parser = TestParser::new(vec![b't', b'r', b'u', b'e']);
    let result = parser.ignore_value();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_ignore_value_invalid_ident() {
    struct TestParser {
        scratch: Vec<u8>,
        input: Vec<u8>,
    }

    impl TestParser {
        fn new(input: Vec<u8>) -> Self {
            Self { scratch: Vec::new(), input }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, Error> {
            if self.input.is_empty() {
                return Ok(None);
            }
            let byte = self.input.remove(0);
            Ok(Some(byte))
        }

        fn eat_char(&mut self) {
            // A no-op for this test as we remove in parse_whitespace
        }

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<(), Error> {
            Err(Error::ParseError)
        }

        fn read_ignore_str(&mut self) -> Result<(), Error> {
            if self.input.get(0) == Some(&b'"') {
                self.input.remove(0);
                Ok(())
            } else {
                Err(Error::ParseError)
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::ParseError
        }
    }

    let mut parser = TestParser::new(vec![b't', b'r', b'u', b'e']);
    parser.ignore_value();
}

#[test]
fn test_ignore_value_eof() {
    struct TestParser {
        scratch: Vec<u8>,
        input: Vec<u8>,
    }

    impl TestParser {
        fn new(input: Vec<u8>) -> Self {
            Self { scratch: Vec::new(), input }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, Error> {
            if self.input.is_empty() {
                return Ok(None);
            }
            let byte = self.input.remove(0);
            Ok(Some(byte))
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<(), Error> {
            Ok(())
        }

        fn read_ignore_str(&mut self) -> Result<(), Error> {
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::ParseError
        }
    }

    let mut parser = TestParser::new(vec![]);
    let result = parser.ignore_value();
    assert!(result.is_err());
}

#[test]
fn test_ignore_value_invalid_eof_in_object() {
    struct TestParser {
        scratch: Vec<u8>,
        input: Vec<u8>,
    }

    impl TestParser {
        fn new(input: Vec<u8>) -> Self {
            Self { scratch: Vec::new(), input }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, Error> {
            if self.input.is_empty() {
                return Ok(None);
            }
            let byte = self.input.remove(0);
            Ok(Some(byte))
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<(), Error> {
            Ok(())
        }

        fn read_ignore_str(&mut self) -> Result<(), Error> {
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::ParseError
        }
    }

    let mut parser = TestParser::new(vec![b'{']);
    let result = parser.ignore_value();
    assert!(result.is_err());
}

