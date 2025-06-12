// Answer 0

#[test]
fn test_description_class_unclosed() {
    let error = Error {
        kind: ErrorKind::ClassUnclosed,
        pattern: String::from("[a-z"),
        span: Span {
            start: Position(0),
            end: Position(4),
        },
    };
    error.description();
}

#[test]
fn test_description_class_unclosed_with_different_pattern() {
    let error = Error {
        kind: ErrorKind::ClassUnclosed,
        pattern: String::from("[A-Z"),
        span: Span {
            start: Position(0),
            end: Position(4),
        },
    };
    error.description();
}

