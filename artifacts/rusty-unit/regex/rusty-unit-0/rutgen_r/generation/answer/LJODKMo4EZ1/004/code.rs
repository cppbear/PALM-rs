// Answer 0

#[derive(Debug)]
enum ErrorKind {
    UnicodeNotAllowed,
    InvalidUtf8,
    UnicodePropertyNotFound,
    UnicodePropertyValueNotFound,
    EmptyClassNotAllowed,
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

#[test]
fn test_unicode_property_not_found() {
    let error_kind = ErrorKind::UnicodePropertyNotFound;
    assert_eq!(error_kind.description(), "Unicode property not found");
}

#[test]
fn test_unicode_not_allowed() {
    let error_kind = ErrorKind::UnicodeNotAllowed;
    assert_ne!(error_kind.description(), "Unicode property not found");
}

#[test]
fn test_invalid_utf8() {
    let error_kind = ErrorKind::InvalidUtf8;
    assert_ne!(error_kind.description(), "Unicode property not found");
}

#[test]
fn test_unicode_property_value_not_found() {
    let error_kind = ErrorKind::UnicodePropertyValueNotFound;
    assert_ne!(error_kind.description(), "Unicode property not found");
}

#[test]
fn test_empty_class_not_allowed() {
    let error_kind = ErrorKind::EmptyClassNotAllowed;
    assert_ne!(error_kind.description(), "Unicode property not found");
}

