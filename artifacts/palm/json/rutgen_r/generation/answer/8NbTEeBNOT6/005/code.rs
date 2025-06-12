// Answer 0

#[derive(Default)]
struct TestParser {
    input: Vec<u8>,
    pos: usize,
}

impl TestParser {
    fn parse_whitespace(&mut self) -> Result<Option<u8>> {
        while self.pos < self.input.len() && self.input[self.pos] == b' ' {
            self.pos += 1;
        }
        if self.pos >= self.input.len() {
            return Ok(None);
        }
        Ok(Some(self.input[self.pos]))
    }
    
    fn eat_char(&mut self) {
        if self.pos < self.input.len() {
            self.pos += 1;
        }
    }

    fn peek_error(&self, _code: ErrorCode) -> Error {
        Error {}
    }
}

/// Error codes as an enumeration for different error types
enum ErrorCode {
    TrailingComma,
    TrailingCharacters,
    EofWhileParsingList,
}

/// Custom error type
struct Error {}

fn end_seq(&mut self) -> Result<()> {
    match tri!(self.parse_whitespace()) {
        Some(b']') => {
            self.eat_char();
            Ok(())
        }
        Some(b',') => {
            self.eat_char();
            match self.parse_whitespace() {
                Ok(Some(b']')) => Err(self.peek_error(ErrorCode::TrailingComma)),
                _ => Err(self.peek_error(ErrorCode::TrailingCharacters)),
            }
        }
        Some(_) => Err(self.peek_error(ErrorCode::TrailingCharacters)),
        None => Err(self.peek_error(ErrorCode::EofWhileParsingList)),
    }
}

#[test]
fn test_end_seq_closing_bracket() {
    let mut parser = TestParser {
        input: vec![b' ', b']'],
        ..Default::default()
    };
    assert_eq!(parser.end_seq(), Ok(()));
}

#[test]
fn test_end_seq_trailing_comma() {
    let mut parser = TestParser {
        input: vec![b' ', b',', b' ', b']'],
        ..Default::default()
    };
    assert_eq!(parser.end_seq(), Err(parser.peek_error(ErrorCode::TrailingComma)));
}

#[test]
fn test_end_seq_trailing_characters() {
    let mut parser = TestParser {
        input: vec![b' ', b'1', b']'],
        ..Default::default()
    };
    assert_eq!(parser.end_seq(), Err(parser.peek_error(ErrorCode::TrailingCharacters)));
}

#[test]
fn test_end_seq_end_of_file() {
    let mut parser = TestParser {
        input: vec![],
        ..Default::default()
    };
    assert_eq!(parser.end_seq(), Err(parser.peek_error(ErrorCode::EofWhileParsingList)));
}

