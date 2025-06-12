// Answer 0

#[test]
fn test_auxiliary_span_flag_repeated_negation() {
    let span = Span { start: Position(0), end: Position(1) };
    let error = Error {
        kind: ErrorKind::FlagRepeatedNegation { original: span },
        pattern: String::from("test pattern"),
        span: span.clone(),
    };
    let _ = error.auxiliary_span();
}

#[test]
fn test_auxiliary_span_flag_duplicate() {
    let span = Span { start: Position(0), end: Position(1) };
    let error = Error {
        kind: ErrorKind::FlagDuplicate { original: span },
        pattern: String::from("test pattern"),
        span: span.clone(),
    };
    let _ = error.auxiliary_span();
}

#[test]
fn test_auxiliary_span_group_name_duplicate() {
    let span = Span { start: Position(0), end: Position(1) };
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate { original: span },
        pattern: String::from("test pattern"),
        span: span.clone(),
    };
    let _ = error.auxiliary_span();
}

#[test]
fn test_auxiliary_span_no_auxiliary() {
    let span = Span { start: Position(0), end: Position(1) };
    let error = Error {
        kind: ErrorKind::ClassUnclosed,
        pattern: String::from("test pattern"),
        span: span.clone(),
    };
    let _ = error.auxiliary_span();
}

