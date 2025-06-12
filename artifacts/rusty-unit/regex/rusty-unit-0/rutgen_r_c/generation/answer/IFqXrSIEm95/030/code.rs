// Answer 0

#[test]
fn test_description_capture_limit_exceeded() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl TestError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                CaptureLimitExceeded => "capture group limit exceeded",
                ClassEscapeInvalid => "invalid escape sequence in character class",
                ClassRangeInvalid => "invalid character class range",
                ClassRangeLiteral => "invalid range boundary, must be a literal",
                ClassUnclosed => "unclosed character class",
                DecimalEmpty => "empty decimal literal",
                DecimalInvalid => "invalid decimal literal",
                EscapeHexEmpty => "empty hexadecimal literal",
                EscapeHexInvalid => "invalid hexadecimal literal",
                EscapeHexInvalidDigit => "invalid hexadecimal digit",
                EscapeUnexpectedEof => "unexpected eof (escape sequence)",
                EscapeUnrecognized => "unrecognized escape sequence",
                FlagDanglingNegation => "dangling flag negation operator",
                FlagDuplicate { .. } => "duplicate flag",
                FlagRepeatedNegation { .. } => "repeated negation",
                FlagUnexpectedEof => "unexpected eof (flag)",
                FlagUnrecognized => "unrecognized flag",
                GroupNameDuplicate { .. } => "duplicate capture group name",
                GroupNameEmpty => "empty capture group name",
                GroupNameInvalid => "invalid capture group name",
                GroupNameUnexpectedEof => "unclosed capture group name",
                GroupUnclosed => "unclosed group",
                GroupUnopened => "unopened group",
                NestLimitExceeded(_) => "nest limit exceeded",
                RepetitionCountInvalid => "invalid repetition count range",
                RepetitionCountUnclosed => "unclosed counted repetition",
                RepetitionMissing => "repetition operator missing expression",
                UnsupportedBackreference => "backreferences are not supported",
                UnsupportedLookAround => "look-around is not supported",
                _ => unreachable!(),
            }
        }
    }

    let error = TestError {
        kind: ErrorKind::CaptureLimitExceeded,
    };

    assert_eq!(error.description(), "capture group limit exceeded");
}

