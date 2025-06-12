// Answer 0

#[test]
fn test_class_range_invalid() {
    let error = Error {
        kind: ErrorKind::ClassRangeInvalid,
        pattern: String::from("a-z"),
        span: Span {
            start: Position(0),
            end: Position(3),
        },
    };
    error.description();
}

#[test]
fn test_class_range_invalid_empty_pattern() {
    let error = Error {
        kind: ErrorKind::ClassRangeInvalid,
        pattern: String::from(""),
        span: Span {
            start: Position(0),
            end: Position(0),
        },
    };
    error.description();
}

#[test]
fn test_class_range_invalid_unclosed_pattern() {
    let error = Error {
        kind: ErrorKind::ClassRangeInvalid,
        pattern: String::from("[a-"),
        span: Span {
            start: Position(0),
            end: Position(4),
        },
    };
    error.description();
}

#[test]
fn test_class_range_invalid_start_greater_than_end() {
    let error = Error {
        kind: ErrorKind::ClassRangeInvalid,
        pattern: String::from("[z-a]"),
        span: Span {
            start: Position(0),
            end: Position(5),
        },
    };
    error.description();
}

#[test]
fn test_class_range_invalid_single_character() {
    let error = Error {
        kind: ErrorKind::ClassRangeInvalid,
        pattern: String::from("[b-b]"),
        span: Span {
            start: Position(0),
            end: Position(5),
        },
    };
    error.description();
}

