// Answer 0

#[derive(Debug)]
struct ErrorKind {
    kind: ErrorType,
}

#[derive(Debug)]
enum ErrorType {
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
    NestLimitExceeded(i32),
    RepetitionCountInvalid,
    RepetitionCountUnclosed,
    RepetitionMissing,
    UnsupportedBackreference,
    UnsupportedLookAround,
}

impl ErrorKind {
    fn description(&self) -> &str {
        use ErrorType::*;
        match self.kind {
            ClassUnclosed => "unclosed character class",
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_description_class_unclosed() {
    let error = ErrorKind { kind: ErrorType::ClassUnclosed };
    assert_eq!(error.description(), "unclosed character class");
}

