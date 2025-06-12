// Answer 0

#[derive(Debug)]
struct MockParser {
    ident: Vec<u8>,
    index: usize,
}

impl MockParser {
    fn new(ident: Vec<u8>) -> Self {
        MockParser { ident, index: 0 }
    }
    
    fn next_char(&mut self) -> Result<Option<u8>, ()> {
        if self.index < self.ident.len() {
            let ch = self.ident[self.index];
            self.index += 1;
            Ok(Some(ch))
        } else {
            Ok(None)
        }
    }
    
    fn error(&self, _: ErrorCode) -> () {
        // Placeholder for error handling
    }
    
    fn parse_ident(&mut self, ident: &[u8]) -> Result<(), ()> {
        for expected in ident {
            match self.next_char() {
                Ok(Some(next)) => {
                    if next != *expected {
                        return Err(self.error(ErrorCode::ExpectedSomeIdent));
                    }
                }
                Ok(None) => {
                    return Err(self.error(ErrorCode::EofWhileParsingValue));
                }
                Err(_) => {
                    return Err(self.error(ErrorCode::EofWhileParsingValue));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
enum ErrorCode {
    EofWhileParsingValue,
    ExpectedSomeIdent,
}

#[test]
fn test_parse_ident_success() {
    let mut parser = MockParser::new(b"test".to_vec());
    let result = parser.parse_ident(b"test");
    assert!(result.is_ok());
}

#[test]
fn test_parse_ident_eof() {
    let mut parser = MockParser::new(b"tes".to_vec());
    let result = parser.parse_ident(b"test");
    assert!(result.is_err());
    // We expect the error to be EofWhileParsingValue
}

#[test]
fn test_parse_ident_expected_ident() {
    let mut parser = MockParser::new(b"tset".to_vec());
    let result = parser.parse_ident(b"test");
    assert!(result.is_err());
    // We expect the error to be ExpectedSomeIdent
}

#[test]
fn test_parse_ident_empty_input() {
    let mut parser = MockParser::new(b"".to_vec());
    let result = parser.parse_ident(b"");
    assert!(result.is_ok());
}

