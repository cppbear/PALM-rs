// Answer 0

#[test]
fn test_auxiliary_span_flag_repeated_negation() {
    let original_span = Span { start: Position(1), end: Position(2) };
    let error = Error {
        kind: ErrorKind::FlagRepeatedNegation { original: original_span },
        pattern: "test_pattern".to_string(),
        span: Span { start: Position(0), end: Position(15) },
    };

    assert_eq!(error.auxiliary_span(), Some(&original_span));
}

#[test]
fn test_auxiliary_span_flag_duplicate() {
    let original_span = Span { start: Position(3), end: Position(4) };
    let error = Error {
        kind: ErrorKind::FlagDuplicate { original: original_span },
        pattern: "test_pattern".to_string(),
        span: Span { start: Position(0), end: Position(15) },
    };

    assert_eq!(error.auxiliary_span(), Some(&original_span));
}

#[test]
fn test_auxiliary_span_group_name_duplicate() {
    let original_span = Span { start: Position(5), end: Position(6) };
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate { original: original_span },
        pattern: "test_pattern".to_string(),
        span: Span { start: Position(0), end: Position(15) },
    };

    assert_eq!(error.auxiliary_span(), Some(&original_span));
}

#[test]
fn test_auxiliary_span_no_auxiliary() {
    let error = Error {
        kind: ErrorKind::ClassUnclosed,
        pattern: "test_pattern".to_string(),
        span: Span { start: Position(0), end: Position(15) },
    };

    assert_eq!(error.auxiliary_span(), None);
}

