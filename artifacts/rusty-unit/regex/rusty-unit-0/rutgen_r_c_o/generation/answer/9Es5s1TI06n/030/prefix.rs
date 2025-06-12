// Answer 0

#[test]
fn test_capture_limit_exceeded() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::CaptureLimitExceeded;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_class_escape_invalid() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::ClassEscapeInvalid;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_class_range_invalid() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::ClassRangeInvalid;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_class_range_literal() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::ClassRangeLiteral;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_class_unclosed() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::ClassUnclosed;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_decimal_empty() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::DecimalEmpty;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_decimal_invalid() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::DecimalInvalid;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_escape_hex_empty() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::EscapeHexEmpty;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_escape_hex_invalid() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::EscapeHexInvalid;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_escape_hex_invalid_digit() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::EscapeHexInvalidDigit;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_escape_unexpected_eof() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::EscapeUnexpectedEof;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_escape_unrecognized() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::EscapeUnrecognized;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_flag_dangling_negation() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::FlagDanglingNegation;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_flag_duplicate() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::FlagDuplicate {
        original: Span { start: 2, end: 3 },
    };
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_flag_repeated_negation() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::FlagRepeatedNegation {
        original: Span { start: 4, end: 5 },
    };
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_flag_unexpected_eof() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::FlagUnexpectedEof;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_flag_unrecognized() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::FlagUnrecognized;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_group_name_duplicate() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::GroupNameDuplicate {
        original: Span { start: 0, end: 1 },
    };
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_group_name_empty() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::GroupNameEmpty;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_group_name_invalid() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::GroupNameInvalid;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_group_name_unexpected_eof() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::GroupNameUnexpectedEof;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_group_unclosed() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::GroupUnclosed;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_group_unopened() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::GroupUnopened;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_nest_limit_exceeded() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::NestLimitExceeded(50);
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_repetition_count_invalid() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::RepetitionCountInvalid;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_repetition_count_unclosed() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::RepetitionCountUnclosed;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_repetition_missing() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::RepetitionMissing;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_unsupported_backreference() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::UnsupportedBackreference;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_unsupported_look_around() {
    use regex_syntax::ErrorKind;
    let error = ErrorKind::UnsupportedLookAround;
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

