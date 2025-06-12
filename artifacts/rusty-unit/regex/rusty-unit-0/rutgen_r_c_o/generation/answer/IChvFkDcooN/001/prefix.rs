// Answer 0

#[test]
fn test_span_valid_range_1() {
    let span = Span { start: Position(0), end: Position(0) };
    let error = Error { kind: ErrorKind::UnicodeNotAllowed, pattern: String::from("test"), span };
    error.span();
}

#[test]
fn test_span_valid_range_2() {
    let span = Span { start: Position(1), end: Position(1) };
    let error = Error { kind: ErrorKind::InvalidUtf8, pattern: String::from("test"), span };
    error.span();
}

#[test]
fn test_span_valid_range_max() {
    let span = Span { start: Position(u32::MAX), end: Position(u32::MAX) };
    let error = Error { kind: ErrorKind::CaptureLimitExceeded, pattern: String::from("test"), span };
    error.span();
}

#[test]
fn test_span_range_non_zero() {
    let span = Span { start: Position(10), end: Position(20) };
    let error = Error { kind: ErrorKind::ClassRangeInvalid, pattern: String::from("test"), span };
    error.span();
}

#[test]
fn test_span_edge_case() {
    let span = Span { start: Position(u32::MAX - 1), end: Position(u32::MAX) };
    let error = Error { kind: ErrorKind::UnicodePropertyNotFound, pattern: String::from("test"), span };
    error.span();
}

