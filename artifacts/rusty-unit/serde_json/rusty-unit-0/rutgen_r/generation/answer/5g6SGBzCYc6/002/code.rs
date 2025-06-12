// Answer 0

#[derive(Debug)]
struct TestParser {
    input: Vec<u8>,
    position: usize,
}

impl TestParser {
    fn next_char(&mut self) -> Result<Option<u8>, ()> {
        if self.position >= self.input.len() {
            return Ok(None);
        }
        let next = self.input[self.position];
        self.position += 1;
        Ok(Some(next))
    }

    fn error(&self, code: ErrorCode) -> () {
        // Simulate error handling
    }

    fn parse_ident(&mut self, ident: &[u8]) -> Result<(), ()> {
        for expected in ident {
            match self.next_char() {
                Ok(None) => {
                    return Err(self.error(ErrorCode::EofWhileParsingValue));
                }
                Ok(Some(next)) => {
                    if next != *expected {
                        return Err(self.error(ErrorCode::ExpectedSomeIdent));
                    }
                }
                Err(_) => {
                    return Err(());
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
    let mut parser = TestParser { input: b"abc".to_vec(), position: 0 };
    let result = parser.parse_ident(b"abc");
    assert!(result.is_ok());
}

#[test]
fn test_parse_ident_eof() {
    let mut parser = TestParser { input: b"ab".to_vec(), position: 0 };
    let result = parser.parse_ident(b"abc");
    assert!(result.is_err());
}

#[test]
fn test_parse_ident_expected_some_ident() {
    let mut parser = TestParser { input: b"abx".to_vec(), position: 0 };
    let result = parser.parse_ident(b"abc");
    assert!(result.is_err());
}

