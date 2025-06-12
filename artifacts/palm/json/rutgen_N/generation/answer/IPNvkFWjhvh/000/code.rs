// Answer 0

#[derive(Debug)]
struct ErrorImpl {
    code: ErrorCode,
    line: usize,
    column: usize,
}

#[derive(Debug)]
struct Error {
    err: Box<ErrorImpl>,
}

#[derive(Debug)]
enum ErrorCode {
    SyntaxError,
    RuntimeError,
}

impl Error {
    pub(crate) fn syntax(code: ErrorCode, line: usize, column: usize) -> Self {
        Error {
            err: Box::new(ErrorImpl { code, line, column }),
        }
    }
}

#[test]
fn test_syntax_creates_error() {
    let error = Error::syntax(ErrorCode::SyntaxError, 1, 5);
    assert_eq!(matches!(error.err.code, ErrorCode::SyntaxError), true);
    assert_eq!(error.err.line, 1);
    assert_eq!(error.err.column, 5);
}

#[test]
fn test_syntax_creates_runtime_error() {
    let error = Error::syntax(ErrorCode::RuntimeError, 10, 20);
    assert_eq!(matches!(error.err.code, ErrorCode::RuntimeError), true);
    assert_eq!(error.err.line, 10);
    assert_eq!(error.err.column, 20);
}

#[test]
fn test_syntax_zero_coordinates() {
    let error = Error::syntax(ErrorCode::SyntaxError, 0, 0);
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
}

