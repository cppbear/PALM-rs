// Answer 0

#[derive(Default)]
struct Parser {
    state: Vec<u8>,
    cursor: usize,
}

impl Parser {
    fn parse_whitespace(&mut self) -> Result<Option<u8>> {
        while self.cursor < self.state.len() {
            let c = self.state[self.cursor];
            if c.is_ascii_whitespace() {
                self.cursor += 1;
            } else {
                return Ok(Some(c));
            }
        }
        Ok(None)
    }

    fn eat_char(&mut self) {
        if self.cursor < self.state.len() {
            self.cursor += 1;
        }
    }

    fn peek_error(&self, error_code: ErrorCode) -> Error {
        Error { code: error_code }
    }
}

#[derive(Debug)]
struct Error {
    code: ErrorCode,
}

#[derive(Debug)]
enum ErrorCode {
    TrailingComma,
    TrailingCharacters,
    EofWhileParsingList,
}

type Result<T> = std::result::Result<T, Error>;

impl Parser {
    fn end_seq(&mut self) -> Result<()> {
        match self.parse_whitespace()? {
            Some(b']') => {
                self.eat_char();
                Ok(())
            }
            Some(b',') => {
                self.eat_char();
                match self.parse_whitespace()? {
                    Some(b']') => Err(self.peek_error(ErrorCode::TrailingComma)),
                    _ => Err(self.peek_error(ErrorCode::TrailingCharacters)),
                }
            }
            Some(_) => Err(self.peek_error(ErrorCode::TrailingCharacters)),
            None => Err(self.peek_error(ErrorCode::EofWhileParsingList)),
        }
    }
}

#[test]
fn test_end_seq_closing_bracket() {
    let mut parser = Parser {
        state: vec![b' ', b']'],
        cursor: 0,
    };
    assert!(parser.end_seq().is_ok());
}

#[test]
fn test_end_seq_with_trailing_comma() {
    let mut parser = Parser {
        state: vec![b',', b' ', b']'],
        cursor: 0,
    };
    assert!(matches!(parser.end_seq(), Err(Error { code: ErrorCode::TrailingComma })));
}

#[test]
fn test_end_seq_with_trailing_characters() {
    let mut parser = Parser {
        state: vec![b'a', b',', b']'],
        cursor: 0,
    };
    assert!(matches!(parser.end_seq(), Err(Error { code: ErrorCode::TrailingCharacters })));
}

#[test]
fn test_end_seq_eof_while_parsing_list() {
    let mut parser = Parser {
        state: vec![],
        cursor: 0,
    };
    assert!(matches!(parser.end_seq(), Err(Error { code: ErrorCode::EofWhileParsingList })));
}

