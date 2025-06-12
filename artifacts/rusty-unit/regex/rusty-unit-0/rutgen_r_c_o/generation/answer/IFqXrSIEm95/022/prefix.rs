// Answer 0

#[test]
fn test_description_escape_hex_invalid() {
    let error = Error {
        kind: ErrorKind::EscapeHexInvalid,
        pattern: String::from("some pattern"),
        span: Span {
            start: Position(0),
            end: Position(10),
        },
    };
    let _ = error.description();
}

#[test]
fn test_description_escape_hex_empty() {
    let error = Error {
        kind: ErrorKind::EscapeHexEmpty,
        pattern: String::from("example"),
        span: Span {
            start: Position(5),
            end: Position(10),
        },
    };
    let _ = error.description();
}

#[test]
fn test_description_escape_unrecognized() {
    let error = Error {
        kind: ErrorKind::EscapeUnrecognized,
        pattern: String::from("test"),
        span: Span {
            start: Position(2),
            end: Position(3),
        },
    };
    let _ = error.description();
}

