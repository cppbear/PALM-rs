// Answer 0

#[test]
fn test_description_capture_limit_exceeded() {
    let error = Error {
        kind: ErrorKind::CaptureLimitExceeded,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "capture group limit exceeded");
}

#[test]
fn test_description_class_escape_invalid() {
    let error = Error {
        kind: ErrorKind::ClassEscapeInvalid,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "invalid escape sequence in character class");
}

#[test]
fn test_description_class_range_invalid() {
    let error = Error {
        kind: ErrorKind::ClassRangeInvalid,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "invalid character class range");
}

#[test]
fn test_description_class_range_literal() {
    let error = Error {
        kind: ErrorKind::ClassRangeLiteral,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "invalid range boundary, must be a literal");
}

#[test]
fn test_description_class_unclosed() {
    let error = Error {
        kind: ErrorKind::ClassUnclosed,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "unclosed character class");
}

#[test]
fn test_description_decimal_empty() {
    let error = Error {
        kind: ErrorKind::DecimalEmpty,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "empty decimal literal");
}

#[test]
fn test_description_decimal_invalid() {
    let error = Error {
        kind: ErrorKind::DecimalInvalid,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "invalid decimal literal");
}

#[test]
fn test_description_escape_hex_empty() {
    let error = Error {
        kind: ErrorKind::EscapeHexEmpty,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "empty hexadecimal literal");
}

#[test]
fn test_description_escape_hex_invalid() {
    let error = Error {
        kind: ErrorKind::EscapeHexInvalid,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "invalid hexadecimal literal");
}

#[test]
fn test_description_escape_hex_invalid_digit() {
    let error = Error {
        kind: ErrorKind::EscapeHexInvalidDigit,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "invalid hexadecimal digit");
}

#[test]
fn test_description_escape_unexpected_eof() {
    let error = Error {
        kind: ErrorKind::EscapeUnexpectedEof,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "unexpected eof (escape sequence)");
}

#[test]
fn test_description_escape_unrecognized() {
    let error = Error {
        kind: ErrorKind::EscapeUnrecognized,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "unrecognized escape sequence");
}

#[test]
fn test_description_flag_dangling_negation() {
    let error = Error {
        kind: ErrorKind::FlagDanglingNegation,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "dangling flag negation operator");
}

#[test]
fn test_description_flag_duplicate() {
    let error = Error {
        kind: ErrorKind::FlagDuplicate { original: Span { start: Position(0), end: Position(1) } },
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "duplicate flag");
}

#[test]
fn test_description_flag_repeated_negation() {
    let error = Error {
        kind: ErrorKind::FlagRepeatedNegation { original: Span { start: Position(0), end: Position(1) } },
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "repeated negation");
}

#[test]
fn test_description_flag_unexpected_eof() {
    let error = Error {
        kind: ErrorKind::FlagUnexpectedEof,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "unexpected eof (flag)");
}

#[test]
fn test_description_flag_unrecognized() {
    let error = Error {
        kind: ErrorKind::FlagUnrecognized,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "unrecognized flag");
}

#[test]
fn test_description_group_name_duplicate() {
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate { original: Span { start: Position(0), end: Position(1) } },
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "duplicate capture group name");
}

#[test]
fn test_description_group_name_empty() {
    let error = Error {
        kind: ErrorKind::GroupNameEmpty,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "empty capture group name");
}

#[test]
fn test_description_group_name_invalid() {
    let error = Error {
        kind: ErrorKind::GroupNameInvalid,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "invalid capture group name");
}

#[test]
fn test_description_group_name_unexpected_eof() {
    let error = Error {
        kind: ErrorKind::GroupNameUnexpectedEof,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "unclosed capture group name");
}

#[test]
fn test_description_group_unclosed() {
    let error = Error {
        kind: ErrorKind::GroupUnclosed,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "unclosed group");
}

#[test]
fn test_description_group_unopened() {
    let error = Error {
        kind: ErrorKind::GroupUnopened,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "unopened group");
}

#[test]
fn test_description_nest_limit_exceeded() {
    let error = Error {
        kind: ErrorKind::NestLimitExceeded(5),
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "nest limit exceeded");
}

#[test]
fn test_description_repetition_count_invalid() {
    let error = Error {
        kind: ErrorKind::RepetitionCountInvalid,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "invalid repetition count range");
}

#[test]
fn test_description_repetition_count_unclosed() {
    let error = Error {
        kind: ErrorKind::RepetitionCountUnclosed,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "unclosed counted repetition");
}

#[test]
fn test_description_repetition_missing() {
    let error = Error {
        kind: ErrorKind::RepetitionMissing,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "repetition operator missing expression");
}

#[test]
fn test_description_unsupported_backreference() {
    let error = Error {
        kind: ErrorKind::UnsupportedBackreference,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "backreferences are not supported");
}

#[test]
fn test_description_unsupported_look_around() {
    let error = Error {
        kind: ErrorKind::UnsupportedLookAround,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    assert_eq!(error.description(), "look-around is not supported");
}

