// Answer 0

#[test]
fn test_error_span() {
    let span = Span { start: Position(0), end: Position(5) };
    let error = Error {
        kind: ErrorKind::CaptureLimitExceeded,
        pattern: "test pattern".to_string(),
        span,
    };
    
    assert_eq!(error.span().start, Position(0));
    assert_eq!(error.span().end, Position(5));
}

#[test]
fn test_error_span_invalid_start() {
    let span = Span { start: Position(10), end: Position(15) };
    let error = Error {
        kind: ErrorKind::ClassRangeInvalid,
        pattern: "sample pattern".to_string(),
        span,
    };

    assert_eq!(error.span().start, Position(10));
    assert_eq!(error.span().end, Position(15));
}

#[test]
fn test_error_span_empty_pattern() {
    let span = Span { start: Position(0), end: Position(0) };
    let error = Error {
        kind: ErrorKind::GroupNameEmpty,
        pattern: "".to_string(),
        span,
    };
    
    assert_eq!(error.span().start, Position(0));
    assert_eq!(error.span().end, Position(0));
}

