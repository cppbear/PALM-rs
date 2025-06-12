// Answer 0

#[test]
fn test_repetition_missing() {
    let span = Span { start: Position(0), end: Position(0) };
    let error = Error {
        kind: ErrorKind::RepetitionMissing,
        pattern: String::from(""),
        span,
    };
    let _description = error.description();
}

#[test]
fn test_repetition_missing_with_non_empty_pattern() {
    let span = Span { start: Position(0), end: Position(1) };
    let error = Error {
        kind: ErrorKind::RepetitionMissing,
        pattern: String::from("abc"),
        span,
    };
    let _description = error.description();
}

#[test]
fn test_repetition_missing_with_different_span() {
    let span = Span { start: Position(3), end: Position(3) };
    let error = Error {
        kind: ErrorKind::RepetitionMissing,
        pattern: String::from("xyz"),
        span,
    };
    let _description = error.description();
}

