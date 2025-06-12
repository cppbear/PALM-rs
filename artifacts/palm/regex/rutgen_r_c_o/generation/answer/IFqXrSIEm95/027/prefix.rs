// Answer 0

#[test]
fn test_description_class_range_literal() {
    let error = Error {
        kind: ErrorKind::ClassRangeLiteral,
        pattern: String::from("a-z"),
        span: Span { start: Position(0), end: Position(3) }
    };
    let result = error.description();
}

#[test]
fn test_description_class_range_literal_alternative() {
    let error = Error {
        kind: ErrorKind::ClassRangeLiteral,
        pattern: String::from("[a-b]"),
        span: Span { start: Position(0), end: Position(5) }
    };
    let result = error.description();
}

#[test]
fn test_description_class_range_literal_empty() {
    let error = Error {
        kind: ErrorKind::ClassRangeLiteral,
        pattern: String::from("[]"),
        span: Span { start: Position(0), end: Position(2) }
    };
    let result = error.description();
}

#[test]
fn test_description_class_range_literal_with_space() {
    let error = Error {
        kind: ErrorKind::ClassRangeLiteral,
        pattern: String::from("a - b"),
        span: Span { start: Position(0), end: Position(5) }
    };
    let result = error.description();
}

#[test]
fn test_description_class_range_literal_single_char() {
    let error = Error {
        kind: ErrorKind::ClassRangeLiteral,
        pattern: String::from("a-a"),
        span: Span { start: Position(0), end: Position(3) }
    };
    let result = error.description();
}

