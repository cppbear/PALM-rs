// Answer 0

#[test]
fn test_flag_repeated_negation_case_1() {
    let start = Position(0);
    let end = Position(1);
    let original_span = Span { start, end };
    let error_kind = ErrorKind::FlagRepeatedNegation { original: original_span };
    let _ = write!(std::fmt::Formatter::new(), "{}", error_kind);
}

#[test]
fn test_flag_repeated_negation_case_2() {
    let start = Position(5);
    let end = Position(10);
    let original_span = Span { start, end };
    let error_kind = ErrorKind::FlagRepeatedNegation { original: original_span };
    let _ = write!(std::fmt::Formatter::new(), "{}", error_kind);
}

#[test]
fn test_flag_repeated_negation_case_3() {
    let start = Position(15);
    let end = Position(20);
    let original_span = Span { start, end };
    let error_kind = ErrorKind::FlagRepeatedNegation { original: original_span };
    let _ = write!(std::fmt::Formatter::new(), "{}", error_kind);
}

