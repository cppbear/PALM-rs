// Answer 0

#[derive(Default)]
struct MockParser {
    input: Vec<u8>,
    index: usize,
}

impl MockParser {
    fn next_char(&mut self) -> Result<Option<u8>, ErrorCode> {
        if self.index < self.input.len() {
            let ch = self.input[self.index];
            self.index += 1;
            Ok(Some(ch))
        } else {
            Ok(None)
        }
    }

    fn error(&self, error: ErrorCode) -> Error {
        Error { code: error }
    }

    fn parse_ident(&mut self, ident: &[u8]) -> Result<(), Error> {
        for expected in ident {
            match self.next_char()? {
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
enum ErrorCode {
    EofWhileParsingValue,
    ExpectedSomeIdent,
}

#[derive(Debug)]
struct Error {
    code: ErrorCode,
}

type Result<T> = core::result::Result<T, Error>;

#[test]
fn test_parse_ident_eof_error() {
    let mut parser = MockParser {
        input: Vec::new(), // Empty input to trigger EOF
        index: 0,
    };
    let result = parser.parse_ident(b"test");
    assert_eq!(result.unwrap_err().code, ErrorCode::EofWhileParsingValue);
}

#[test]
fn test_parse_ident_expected_error() {
    let mut parser = MockParser {
        input: b"tesz".to_vec(), // Input that does not match expected
        index: 0,
    };
    let result = parser.parse_ident(b"test");
    assert_eq!(result.unwrap_err().code, ErrorCode::ExpectedSomeIdent);
}

#[test]
fn test_parse_ident_success() {
    let mut parser = MockParser {
        input: b"test".to_vec(), // Valid input that matches expected
        index: 0,
    };
    let result = parser.parse_ident(b"test");
    assert!(result.is_ok());
}

