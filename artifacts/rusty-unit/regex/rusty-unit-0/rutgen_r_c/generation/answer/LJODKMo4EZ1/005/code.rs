// Answer 0

#[test]
fn test_description_invalid_utf8() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ErrorKind {
        UnicodeNotAllowed,
        InvalidUtf8,
        UnicodePropertyNotFound,
        UnicodePropertyValueNotFound,
        EmptyClassNotAllowed,
        __Nonexhaustive,
    }

    impl ErrorKind {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match *self {
                UnicodeNotAllowed => "Unicode not allowed here",
                InvalidUtf8 => "pattern can match invalid UTF-8",
                UnicodePropertyNotFound => "Unicode property not found",
                UnicodePropertyValueNotFound => "Unicode property value not found",
                EmptyClassNotAllowed => "empty character classes are not allowed",
                _ => unreachable!(),
            }
        }
    }

    let error = ErrorKind::InvalidUtf8;
    assert_eq!(error.description(), "pattern can match invalid UTF-8");
}

