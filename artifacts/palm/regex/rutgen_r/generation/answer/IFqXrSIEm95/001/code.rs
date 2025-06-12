// Answer 0

#[derive(Debug)]
struct ErrorKind {
    kind: ErrorKindVariant,
}

#[derive(Debug)]
enum ErrorKindVariant {
    CaptureLimitExceeded,
    ClassEscapeInvalid,
    ClassRangeInvalid,
    ClassRangeLiteral,
    ClassUnclosed,
    DecimalEmpty,
    DecimalInvalid,
    EscapeHexEmpty,
    EscapeHexInvalid,
    EscapeHexInvalidDigit,
    EscapeUnexpectedEof,
    EscapeUnrecognized,
    FlagDanglingNegation,
    FlagDuplicate,
    FlagRepeatedNegation,
    FlagUnexpectedEof,
    FlagUnrecognized,
    GroupNameDuplicate,
    GroupNameEmpty,
    GroupNameInvalid,
    GroupNameUnexpectedEof,
    GroupUnclosed,
    GroupUnopened,
    NestLimitExceeded(u32),
    RepetitionCountInvalid,
    RepetitionCountUnclosed,
    RepetitionMissing,
    UnsupportedBackreference,
    UnsupportedLookAround,
}

impl ErrorKind {
    fn description(&self) -> &str {
        use ErrorKindVariant::*;
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
            FlagDuplicate => "duplicate flag",
            FlagRepeatedNegation => "repeated negation",
            FlagUnexpectedEof => "unexpected eof (flag)",
            FlagUnrecognized => "unrecognized flag",
            GroupNameDuplicate => "duplicate capture group name",
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

#[test]
fn test_description_for_variant() {
    let error = ErrorKind { kind: ErrorKindVariant::FlagUnrecognized };
    assert_eq!(error.description(), "unrecognized flag");

    let error = ErrorKind { kind: ErrorKindVariant::InvalidRangeBoundary };
    assert_eq!(error.description(), "invalid range boundary, must be a literal");
    
    let error = ErrorKind { kind: ErrorKindVariant::RepetitionCountInvalid };
    assert_eq!(error.description(), "invalid repetition count range");
}

#[test]
#[should_panic]
fn test_description_unreachable() {
    let error = ErrorKind { kind: ErrorKindVariant::CaptureLimitExceeded };
    assert_eq!(error.description(), "capture group limit exceeded");
}

