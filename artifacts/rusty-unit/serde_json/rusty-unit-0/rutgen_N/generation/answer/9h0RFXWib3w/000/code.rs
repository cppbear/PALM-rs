// Answer 0

#[derive(Debug)]
struct Read {
    position: Position,
}

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

impl Read {
    fn peek_position(&self) -> Position {
        self.position.clone()
    }
}

#[derive(Debug)]
enum ErrorCode {
    InvalidByte,
    UnexpectedEnd,
}

#[derive(Debug)]
struct Error {
    code: ErrorCode,
    line: usize,
    column: usize,
}

impl Error {
    fn syntax(code: ErrorCode, line: usize, column: usize) -> Error {
        Error { code, line, column }
    }
}

fn peek_error(read: &Read, reason: ErrorCode) -> Error {
    let position = read.peek_position();
    Error::syntax(reason, position.line, position.column)
}

#[test]
fn test_peek_error_invalid_byte() {
    let read = Read { position: Position { line: 10, column: 5 } };
    let error = peek_error(&read, ErrorCode::InvalidByte);
    assert_eq!(error.line, 10);
    assert_eq!(error.column, 5);
    assert_eq!(error.code, ErrorCode::InvalidByte);
}

#[test]
fn test_peek_error_unexpected_end() {
    let read = Read { position: Position { line: 20, column: 15 } };
    let error = peek_error(&read, ErrorCode::UnexpectedEnd);
    assert_eq!(error.line, 20);
    assert_eq!(error.column, 15);
    assert_eq!(error.code, ErrorCode::UnexpectedEnd);
}

