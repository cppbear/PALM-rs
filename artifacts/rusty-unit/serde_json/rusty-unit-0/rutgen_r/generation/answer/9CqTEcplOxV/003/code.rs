// Answer 0

fn test_parse_object_colon_whitespace_ok_colon() {
    struct Parser {
        input: Vec<u8>,
        position: usize,
    }
    
    impl Parser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            while self.position < self.input.len() && self.input[self.position].is_ascii_whitespace() {
                self.position += 1;
            }
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }
        
        fn eat_char(&mut self) {
            self.position += 1;
        }
        
        fn peek_error(&self, error_code: ErrorCode) -> Error {
            Error { code: error_code }
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
        position: 0 
    };
    assert_eq!(parser.parse_object_colon(), Ok(()));
}

fn test_parse_object_colon_whitespace_err() {
    struct Parser {
        input: Vec<u8>,
        position: usize,
    }
    
    impl Parser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            while self.position < self.input.len() && self.input[self.position].is_ascii_whitespace() {
                self.position += 1;
            }
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }
        
        fn eat_char(&mut self) {
            self.position += 1;
        }
        
        fn peek_error(&self, error_code: ErrorCode) -> Error {
            Error { code: error_code }
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
        input: b" ; ".to_vec(), 
        position: 0 
    };
    assert_eq!(parser.parse_object_colon(), Err(Error { code: ErrorCode::ExpectedColon }));
}

fn test_parse_object_colon_eof() {
    struct Parser {
        input: Vec<u8>,
        position: usize,
    }
    
    impl Parser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            while self.position < self.input.len() && self.input[self.position].is_ascii_whitespace() {
                self.position += 1;
            }
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }
        
        fn eat_char(&mut self) {
            self.position += 1;
        }
        
        fn peek_error(&self, error_code: ErrorCode) -> Error {
            Error { code: error_code }
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
        input: b"".to_vec(), 
        position: 0 
    };
    assert_eq!(parser.parse_object_colon(), Err(Error { code: ErrorCode::EofWhileParsingObject }));
}

