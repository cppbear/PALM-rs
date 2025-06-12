// Answer 0

#[test]
fn test_error_kind_flag_unexpected_eof() {
    use std::fmt::Write;

    let error_kind = ErrorKind::FlagUnexpectedEof;

    let mut output = String::new();
    let result = error_kind.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "expected flag but got end of regex");
}

#[test]
fn test_error_kind_flag_repeated_negation() {
    use std::fmt::Write;

    let error_kind = ErrorKind::FlagRepeatedNegation {
        original: Span { start: Position::default(), end: Position::default() },
    };

    let mut output = String::new();
    let result = error_kind.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "flag negation operator repeated");
}

#[test]
fn test_error_kind_flag_duplicate() {
    use std::fmt::Write;

    let error_kind = ErrorKind::FlagDuplicate {
        original: Span { start: Position::default(), end: Position::default() },
    };

    let mut output = String::new();
    let result = error_kind.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "duplicate flag");
}

