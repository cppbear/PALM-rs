// Answer 0

#[test]
fn test_error_kind_display() {
    use std::fmt::Write; // Importing the Write trait for the write! macro already in use

    let cases = vec![
        (ErrorKind::CaptureLimitExceeded, "exceeded the maximum number of capturing groups (4294967295)"),
        (ErrorKind::ClassEscapeInvalid, "invalid escape sequence found in character class"),
        (ErrorKind::ClassRangeInvalid, "invalid character class range, the start must be <= the end"),
        (ErrorKind::ClassRangeLiteral, "invalid range boundary, must be a literal"),
        (ErrorKind::ClassUnclosed, "unclosed character class"),
        (ErrorKind::DecimalEmpty, "decimal literal empty"),
        (ErrorKind::DecimalInvalid, "decimal literal invalid"),
        (ErrorKind::EscapeHexEmpty, "hexadecimal literal empty"),
        (ErrorKind::EscapeHexInvalid, "hexadecimal literal is not a Unicode scalar value"),
        (ErrorKind::EscapeHexInvalidDigit, "invalid hexadecimal digit"),
        (ErrorKind::EscapeUnexpectedEof, "incomplete escape sequence, reached end of pattern prematurely"),
        (ErrorKind::EscapeUnrecognized, "unrecognized escape sequence"),
        (ErrorKind::FlagDanglingNegation, "dangling flag negation operator"),
        (ErrorKind::FlagUnexpectedEof, "expected flag but got end of regex"),
        (ErrorKind::FlagUnrecognized, "unrecognized flag"),
        (ErrorKind::GroupNameEmpty, "empty capture group name"),
        (ErrorKind::GroupNameInvalid, "invalid capture group character"),
        (ErrorKind::GroupNameUnexpectedEof, "unclosed capture group name"),
        (ErrorKind::GroupUnclosed, "unclosed group"),
        (ErrorKind::GroupUnopened, "unopened group"),
        (ErrorKind::NestLimitExceeded(5), "exceed the maximum number of nested parentheses/brackets (5)"),
        (ErrorKind::RepetitionCountInvalid, "invalid repetition count range, the start must be <= the end"),
        (ErrorKind::RepetitionCountUnclosed, "unclosed counted repetition"),
        (ErrorKind::RepetitionMissing, "repetition operator missing expression"),
        (ErrorKind::UnsupportedBackreference, "backreferences are not supported"),
        (ErrorKind::UnsupportedLookAround, "look-around, including look-ahead and look-behind, is not supported"),
    ];

    for (kind, expected) in cases {
        let mut buffer = String::new();
        write!(&mut buffer, "{}", kind).unwrap();
        assert_eq!(buffer, expected);
    }
}

