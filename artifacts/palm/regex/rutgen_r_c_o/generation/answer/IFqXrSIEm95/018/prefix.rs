// Answer 0

#[test]
fn test_description_flag_dangling_negation() {
    use regex_syntax::ErrorKind;
    use regex_syntax::Error;
    use regex_syntax::Span;

    let span = Span { start: 0, end: 0 };
    let error = Error {
        kind: ErrorKind::FlagDanglingNegation,
        pattern: "test pattern".to_string(),
        span,
    };

    let _ = error.description();
}

