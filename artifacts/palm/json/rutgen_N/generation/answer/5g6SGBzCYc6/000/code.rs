// Answer 0

#[test]
fn test_parse_ident_matching_ident() {
    struct TestParser {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &[u8]) -> Self {
            Self { input: input.to_vec(), pos: 0 }
        }

        fn next_char(&mut self) -> Option<u8> {
            if self.pos < self.input.len() {
                let ch = self.input[self.pos];
                self.pos += 1;
                Some(ch)
            } else {
                None
            }
        }

        fn error(&self, _code: ErrorCode) -> Error {
            Error {}
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            for expected in ident {
                match self.next_char() {
                    None => {
                        return Err(self.error(ErrorCode::EofWhileParsingValue));
                    }
                    Some(next) => {
                        if next != *expected {
                            return Err(self.error(ErrorCode::ExpectedSomeIdent));
                        }
                    }
                }
            }
            Ok(())
        }
    }

    let mut parser = TestParser::new(b"abc");
    let result = parser.parse_ident(b"abc");
    assert!(result.is_ok());
}

#[test]
fn test_parse_ident_eof_error() {
    struct TestParser {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &[u8]) -> Self {
            Self { input: input.to_vec(), pos: 0 }
        }

        fn next_char(&mut self) -> Option<u8> {
            if self.pos < self.input.len() {
                let ch = self.input[self.pos];
                self.pos += 1;
                Some(ch)
            } else {
                None
            }
        }

        fn error(&self, _code: ErrorCode) -> Error {
            Error {}
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            for expected in ident {
                match self.next_char() {
                    None => {
                        return Err(self.error(ErrorCode::EofWhileParsingValue));
                    }
                    Some(next) => {
                        if next != *expected {
                            return Err(self.error(ErrorCode::ExpectedSomeIdent));
                        }
                    }
                }
            }
            Ok(())
        }
    }

    let mut parser = TestParser::new(b"ab");
    let result = parser.parse_ident(b"abc");
    assert!(result.is_err());
}

#[test]
fn test_parse_ident_mismatching_ident() {
    struct TestParser {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &[u8]) -> Self {
            Self { input: input.to_vec(), pos: 0 }
        }

        fn next_char(&mut self) -> Option<u8> {
            if self.pos < self.input.len() {
                let ch = self.input[self.pos];
                self.pos += 1;
                Some(ch)
            } else {
                None
            }
        }

        fn error(&self, _code: ErrorCode) -> Error {
            Error {}
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            for expected in ident {
                match self.next_char() {
                    None => {
                        return Err(self.error(ErrorCode::EofWhileParsingValue));
                    }
                    Some(next) => {
                        if next != *expected {
                            return Err(self.error(ErrorCode::ExpectedSomeIdent));
                        }
                    }
                }
            }
            Ok(())
        }
    }

    let mut parser = TestParser::new(b"abd");
    let result = parser.parse_ident(b"abc");
    assert!(result.is_err());
}

