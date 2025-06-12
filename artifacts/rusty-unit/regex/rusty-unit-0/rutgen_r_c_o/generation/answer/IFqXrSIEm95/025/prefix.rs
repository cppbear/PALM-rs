// Answer 0

#[test]
fn test_description_decimal_empty() {
    let error = Error {
        kind: ErrorKind::DecimalEmpty,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    let _ = error.description();
}

#[test]
fn test_description_invalid_escape_sequence() {
    let error = Error {
        kind: ErrorKind::ClassEscapeInvalid,
        pattern: String::from("a\\b"),
        span: Span { start: Position(1), end: Position(3) },
    };
    let _ = error.description();
}

#[test]
fn test_description_unclosed_character_class() {
    let error = Error {
        kind: ErrorKind::ClassUnclosed,
        pattern: String::from("[a-z"),
        span: Span { start: Position(0), end: Position(4) },
    };
    let _ = error.description();
}

