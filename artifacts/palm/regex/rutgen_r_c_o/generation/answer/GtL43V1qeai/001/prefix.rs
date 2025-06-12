// Answer 0

#[test]
fn test_kind_capture_limit_exceeded() {
    let error = Error {
        kind: ErrorKind::CaptureLimitExceeded,
        pattern: String::from("(?P<name>abc)"),
        span: Span { start: Position::new(0), end: Position::new(14) },
    };
    let _ = error.kind();
}

#[test]
fn test_kind_class_escape_invalid() {
    let error = Error {
        kind: ErrorKind::ClassEscapeInvalid,
        pattern: String::from("[a-z\\]"),
        span: Span { start: Position::new(0), end: Position::new(14) },
    };
    let _ = error.kind();
}

#[test]
fn test_kind_class_range_invalid() {
    let error = Error {
        kind: ErrorKind::ClassRangeInvalid,
        pattern: String::from("[z-a]"),
        span: Span { start: Position::new(0), end: Position::new(14) },
    };
    let _ = error.kind();
}

#[test]
fn test_kind_group_name_duplicate() {
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate { original: Span { start: Position::new(0), end: Position::new(5) } },
        pattern: String::from("(?P<name>abc)(?P<name>def)"),
        span: Span { start: Position::new(0), end: Position::new(14) },
    };
    let _ = error.kind();
}

#[test]
fn test_kind_nest_limit_exceeded() {
    let error = Error {
        kind: ErrorKind::NestLimitExceeded(10),
        pattern: String::from("((((((((((((((()))))))))))))))))"),
        span: Span { start: Position::new(0), end: Position::new(14) },
    };
    let _ = error.kind();
}

#[test]
fn test_kind_empty_class_not_allowed() {
    let error = Error {
        kind: ErrorKind::EmptyClassNotAllowed,
        pattern: String::from("[]"),
        span: Span { start: Position::new(0), end: Position::new(14) },
    };
    let _ = error.kind();
}

#[test]
fn test_kind_flag_unrecognized() {
    let error = Error {
        kind: ErrorKind::FlagUnrecognized,
        pattern: String::from("(?a)"),
        span: Span { start: Position::new(0), end: Position::new(14) },
    };
    let _ = error.kind();
}

#[test]
fn test_kind_repetition_count_invalid() {
    let error = Error {
        kind: ErrorKind::RepetitionCountInvalid,
        pattern: String::from("{2,1}"),
        span: Span { start: Position::new(0), end: Position::new(14) },
    };
    let _ = error.kind();
}

