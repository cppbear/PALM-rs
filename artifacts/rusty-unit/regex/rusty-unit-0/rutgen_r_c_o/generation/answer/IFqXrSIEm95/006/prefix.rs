// Answer 0

#[test]
fn test_description_repetition_count_invalid() {
    let kind = ErrorKind::RepetitionCountInvalid;
    let error = Error { kind, pattern: String::from("x{2,1}"), span: Span { start: Position(0), end: Position(10) } };
    error.description();
}

#[test]
fn test_description_repetition_count_unclosed() {
    let kind = ErrorKind::RepetitionCountUnclosed;
    let error = Error { kind, pattern: String::from("x{2,3"), span: Span { start: Position(0), end: Position(8) } };
    error.description();
}

#[test]
fn test_description_repetition_missing() {
    let kind = ErrorKind::RepetitionMissing;
    let error = Error { kind, pattern: String::from("*"), span: Span { start: Position(0), end: Position(1) } };
    error.description();
}

