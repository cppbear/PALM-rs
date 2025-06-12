// Answer 0

#[test]
fn test_fmt_repetition_count_invalid() {
    use std::fmt::Write;

    let error = ErrorKind::RepetitionCountInvalid;

    let mut output = String::new();
    let result = error.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "invalid repetition count range, \
                         the start must be <= the end");
}

#[test]
fn test_fmt_repetition_count_unclosed() {
    use std::fmt::Write;

    let error = ErrorKind::RepetitionCountUnclosed;

    let mut output = String::new();
    let result = error.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "unclosed counted repetition");
}

