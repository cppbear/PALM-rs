// Answer 0

#[test]
fn test_parse_ident_success() {
    struct Parser {
        chars: Vec<u8>,
        pos: usize,
    }

    impl Parser {
        fn new(chars: &[u8]) -> Self {
            Self {
                chars: chars.to_vec(),
                pos: 0,
            }
        }

        fn next_char(&mut self) -> Option<u8> {
            if self.pos < self.chars.len() {
                let ch = self.chars[self.pos];
                self.pos += 1;
                Some(ch)
            } else {
                None
            }
        }

        fn error(&self, code: ErrorCode) -> Error {
            Error {
                code,
                message: "error".to_string(),
            }
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

    #[derive(Debug)]
    struct Error {
        code: ErrorCode,
        message: String,
    }

    #[derive(Debug)]
    enum ErrorCode {
        EofWhileParsingValue,
        ExpectedSomeIdent,
    }

    type Result<T> = std::result::Result<T, Error>;

    let mut parser = Parser::new(b"abc");
    let result = parser.parse_ident(b"abc");
    assert_eq!(result, Ok(()));
}

#[test]
fn test_parse_ident_eof() {
    struct Parser {
        chars: Vec<u8>,
        pos: usize,
    }

    impl Parser {
        fn new(chars: &[u8]) -> Self {
            Self {
                chars: chars.to_vec(),
                pos: 0,
            }
        }

        fn next_char(&mut self) -> Option<u8> {
            if self.pos < self.chars.len() {
                let ch = self.chars[self.pos];
                self.pos += 1;
                Some(ch)
            } else {
                None
            }
        }

        fn error(&self, code: ErrorCode) -> Error {
            Error {
                code,
                message: "error".to_string(),
            }
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

    #[derive(Debug)]
    struct Error {
        code: ErrorCode,
        message: String,
    }

    #[derive(Debug)]
    enum ErrorCode {
        EofWhileParsingValue,
        ExpectedSomeIdent,
    }

    type Result<T> = std::result::Result<T, Error>;

    let mut parser = Parser::new(b"ab");
    let result = parser.parse_ident(b"abc");
    assert_eq!(result, Err(Error { code: ErrorCode::EofWhileParsingValue, message: "error".to_string() }));
}

#[test]
fn test_parse_ident_unexpected_char() {
    struct Parser {
        chars: Vec<u8>,
        pos: usize,
    }

    impl Parser {
        fn new(chars: &[u8]) -> Self {
            Self {
                chars: chars.to_vec(),
                pos: 0,
            }
        }

        fn next_char(&mut self) -> Option<u8> {
            if self.pos < self.chars.len() {
                let ch = self.chars[self.pos];
                self.pos += 1;
                Some(ch)
            } else {
                None
            }
        }

        fn error(&self, code: ErrorCode) -> Error {
            Error {
                code,
                message: "error".to_string(),
            }
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

    #[derive(Debug)]
    struct Error {
        code: ErrorCode,
        message: String,
    }

    #[derive(Debug)]
    enum ErrorCode {
        EofWhileParsingValue,
        ExpectedSomeIdent,
    }

    type Result<T> = std::result::Result<T, Error>;

    let mut parser = Parser::new(b"acd");
    let result = parser.parse_ident(b"abc");
    assert_eq!(result, Err(Error { code: ErrorCode::ExpectedSomeIdent, message: "error".to_string() }));
}

