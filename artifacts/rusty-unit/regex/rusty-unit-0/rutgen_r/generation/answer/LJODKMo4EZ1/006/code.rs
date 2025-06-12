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
fn test_unicode_not_allowed_description() {
    let error = ErrorKind::UnicodeNotAllowed;
    assert_eq!(error.description(), "Unicode not allowed here");
}

#[test]
fn test_invalid_utf8_description() {
    let error = ErrorKind::InvalidUtf8;
    assert_eq!(error.description(), "pattern can match invalid UTF-8");
}

#[test]
fn test_unicode_property_not_found_description() {
    let error = ErrorKind::UnicodePropertyNotFound;
    assert_eq!(error.description(), "Unicode property not found");
}

#[test]
fn test_unicode_property_value_not_found_description() {
    let error = ErrorKind::UnicodePropertyValueNotFound;
    assert_eq!(error.description(), "Unicode property value not found");
}

#[test]
fn test_empty_class_not_allowed_description() {
    let error = ErrorKind::EmptyClassNotAllowed;
    assert_eq!(error.description(), "empty character classes are not allowed");
}

