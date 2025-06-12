// Answer 0

#[test]
fn test_description_unicode_property_value_not_found() {
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

    let error_kind = ErrorKind::UnicodePropertyValueNotFound;
    assert_eq!(error_kind.description(), "Unicode property value not found");
}

#[test]
fn test_description_unicode_property_not_found() {
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

    let error_kind = ErrorKind::UnicodePropertyNotFound;
    assert_eq!(error_kind.description(), "Unicode property not found");
}

