// Answer 0

#[test]
fn test_description_flag_unexpected_eof() {
    let error = Error {
        kind: ErrorKind::FlagUnexpectedEof,
        pattern: String::from("(?"),
        span: Span {
            start: Position::new(0),
            end: Position::new(1),
        },
    };
    let _ = error.description();
}

#[test]
fn test_description_flag_unexpected_eof_with_empty_pattern() {
    let error = Error {
        kind: ErrorKind::FlagUnexpectedEof,
        pattern: String::from(""),
        span: Span {
            start: Position::new(0),
            end: Position::new(0),
        },
    };
    let _ = error.description();
}

