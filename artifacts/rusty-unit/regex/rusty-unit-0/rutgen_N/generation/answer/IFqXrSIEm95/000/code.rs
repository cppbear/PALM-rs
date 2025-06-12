// Answer 0

#[derive(Debug)]
struct ErrorKind {
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
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
    NestLimitExceeded(usize),
    RepetitionCountInvalid,
    RepetitionCountUnclosed,
    RepetitionMissing,
    UnsupportedBackreference,
    UnsupportedLookAround,
}

impl ErrorKind {
    fn description(&self) -> &str {
        use Kind::*;
        match &self.kind {
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
fn test_capture_limit_exceeded_description() {
    let error = ErrorKind { kind: Kind::CaptureLimitExceeded };
    assert_eq!(error.description(), "capture group limit exceeded");
}

#[test]
fn test_class_escape_invalid_description() {
    let error = ErrorKind { kind: Kind::ClassEscapeInvalid };
    assert_eq!(error.description(), "invalid escape sequence in character class");
}

#[test]
fn test_class_range_invalid_description() {
    let error = ErrorKind { kind: Kind::ClassRangeInvalid };
    assert_eq!(error.description(), "invalid character class range");
}

#[test]
fn test_decimal_empty_description() {
    let error = ErrorKind { kind: Kind::DecimalEmpty };
    assert_eq!(error.description(), "empty decimal literal");
}

#[test]
fn test_flag_duplicate_description() {
    let error = ErrorKind { kind: Kind::FlagDuplicate };
    assert_eq!(error.description(), "duplicate flag");
}

#[test]
fn test_nest_limit_exceeded_description() {
    let error = ErrorKind { kind: Kind::NestLimitExceeded(10) };
    assert_eq!(error.description(), "nest limit exceeded");
}

