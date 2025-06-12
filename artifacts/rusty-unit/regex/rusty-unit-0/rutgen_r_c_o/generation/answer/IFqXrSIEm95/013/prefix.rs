// Answer 0

#[test]
fn test_group_name_duplicate() {
    let span = Span { start: Position(0), end: Position(10) };
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate { original: span },
    };
    error.description();
}

#[test]
fn test_group_name_duplicate_with_different_span() {
    let span = Span { start: Position(1), end: Position(5) };
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate { original: span },
    };
    error.description();
}

#[test]
fn test_group_name_duplicate_with_empty_span() {
    let span = Span { start: Position(0), end: Position(0) };
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate { original: span },
    };
    error.description();
}

#[test]
fn test_group_name_duplicate_with_large_span() {
    let span = Span { start: Position(100), end: Position(200) };
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate { original: span },
    };
    error.description();
}

