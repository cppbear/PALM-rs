// Answer 0

#[test]
fn test_description_group_name_empty() {
    let error = Error {
        kind: ErrorKind::GroupNameEmpty,
        pattern: String::from("(?P<>abc)"),
        span: Span { start: Position(0), end: Position(7) },
    };
    error.description();
}

#[test]
fn test_description_group_name_duplicate() {
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate { original: Span { start: Position(0), end: Position(1) } },
        pattern: String::from("(?P<name>abc)(?P<name>def)"),
        span: Span { start: Position(0), end: Position(25) },
    };
    error.description();
}

#[test]
fn test_description_group_unclosed() {
    let error = Error {
        kind: ErrorKind::GroupUnclosed,
        pattern: String::from("(abc"),
        span: Span { start: Position(0), end: Position(4) },
    };
    error.description();
}

