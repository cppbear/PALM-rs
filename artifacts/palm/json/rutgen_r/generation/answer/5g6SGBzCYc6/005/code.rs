// Answer 0

#[test]
fn test_parse_ident_success() {
    struct MockParser {
        data: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(data: Vec<u8>) -> Self {
            MockParser { data, index: 0 }
        }

        fn next_char(&mut self) -> Option<u8> {
            if self.index < self.data.len() {
                let ch = self.data[self.index];
                self.index += 1;
                Some(ch)
            } else {
                None
            }
        }

        fn error(&self, code: ErrorCode) -> Error {
            Error { code }
        }
    }

    #[derive(Debug)]
    struct Error {
        code: ErrorCode,
    }

    #[derive(Debug)]
    enum ErrorCode {
        EofWhileParsingValue,
        ExpectedSomeIdent,
    }

    let mut parser = MockParser::new(vec![b'a', b'b', b'c']); // valid input sequence
    let result = parser.parse_ident(&[b'a', b'b', b'c']);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "EofWhileParsingValue")]
fn test_parse_ident_eof_error() {
    struct MockParser {
        data: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(data: Vec<u8>) -> Self {
            MockParser { data, index: 0 }
        }

        fn next_char(&mut self) -> Option<u8> {
            if self.index < self.data.len() {
                let ch = self.data[self.index];
                self.index += 1;
                Some(ch)
            } else {
                None
            }
        }

        fn error(&self, code: ErrorCode) -> Error {
            Error { code }
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<(), Error> {
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

    #[derive(Debug)]
    struct Error {
        code: ErrorCode,
    }

    #[derive(Debug)]
    enum ErrorCode {
        EofWhileParsingValue,
        ExpectedSomeIdent,
    }

    let mut parser = MockParser::new(vec![b'a']); // insufficient input
    parser.parse_ident(&[b'a', b'b']); // should panic due to EOF
}

#[test]
#[should_panic(expected = "ExpectedSomeIdent")]
fn test_parse_ident_unexpected_char() {
    struct MockParser {
        data: Vec<u8>,
        index: usize,
    }

    impl MockParser {
        fn new(data: Vec<u8>) -> Self {
            MockParser { data, index: 0 }
        }

        fn next_char(&mut self) -> Option<u8> {
            if self.index < self.data.len() {
                let ch = self.data[self.index];
                self.index += 1;
                Some(ch)
            } else {
                None
            }
        }

        fn error(&self, code: ErrorCode) -> Error {
            Error { code }
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<(), Error> {
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

    #[derive(Debug)]
    struct Error {
        code: ErrorCode,
    }

    #[derive(Debug)]
    enum ErrorCode {
        EofWhileParsingValue,
        ExpectedSomeIdent,
    }

    let mut parser = MockParser::new(vec![b'a', b'x']); // unexpected character 'x'
    parser.parse_ident(&[b'a', b'b']); // should panic due to unexpected character
}

