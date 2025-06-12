// Answer 0

#[test]
fn test_parse_object_colon_valid_colon() {
    struct Parser {
        input: Vec<u8>,
        position: usize,
    }

    impl Parser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            for i in self.position..self.input.len() {
                match self.input[i] {
                    b' ' | b'\t' | b'\n' | b'\r' => continue,
                    b':' => {
                        self.position = i;
                        return Ok(Some(b':'));
                    }
                    _ => {
                        self.position = i;
                        return Ok(Some(self.input[i]));
                    }
                }
            }
            Ok(None)
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }
        
        fn peek_error(&self, error: ErrorCode) -> Error {
            Error::new(error)
        }

        fn parse_object_colon(&mut self) -> Result<(), Error> {
            match self.parse_whitespace()? {
                Some(b':') => {
                    self.eat_char();
                    Ok(())
                }
                Some(_) => Err(self.peek_error(ErrorCode::ExpectedColon)),
                None => Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
            }
        }
    }

    let mut parser = Parser {
        input: b" : ".to_vec(),
        position: 0,
    };
    
    assert!(parser.parse_object_colon().is_ok());
}

#[test]
fn test_parse_object_colon_invalid_no_colon() {
    struct Parser {
        input: Vec<u8>,
        position: usize,
    }

    impl Parser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            for i in self.position..self.input.len() {
                match self.input[i] {
                    b' ' | b'\t' | b'\n' | b'\r' => continue,
                    _ => {
                        self.position = i;
                        return Ok(Some(self.input[i]));
                    }
                }
            }
            Ok(None)
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn peek_error(&self, error: ErrorCode) -> Error {
            Error::new(error)
        }

        fn parse_object_colon(&mut self) -> Result<(), Error> {
            match self.parse_whitespace()? {
                Some(b':') => {
                    self.eat_char();
                    Ok(())
                }
                Some(_) => Err(self.peek_error(ErrorCode::ExpectedColon)),
                None => Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
            }
        }
    }

    let mut parser = Parser {
        input: b" = ".to_vec(),
        position: 0,
    };

    assert!(parser.parse_object_colon().is_err());
}

#[test]
fn test_parse_object_colon_eof() {
    struct Parser {
        input: Vec<u8>,
        position: usize,
    }

    impl Parser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            for i in self.position..self.input.len() {
                match self.input[i] {
                    b' ' | b'\t' | b'\n' | b'\r' => continue,
                    _ => {
                        self.position = i;
                        return Ok(Some(self.input[i]));
                    }
                }
            }
            Ok(None)
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn peek_error(&self, error: ErrorCode) -> Error {
            Error::new(error)
        }

        fn parse_object_colon(&mut self) -> Result<(), Error> {
            match self.parse_whitespace()? {
                Some(b':') => {
                    self.eat_char();
                    Ok(())
                }
                Some(_) => Err(self.peek_error(ErrorCode::ExpectedColon)),
                None => Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
            }
        }
    }

    let mut parser = Parser {
        input: b" ".to_vec(),
        position: 0,
    };

    assert!(parser.parse_object_colon().is_err());
}

