// Answer 0

#[derive(Debug)]
enum ErrorKind {
    UnicodeNotAllowed,
    InvalidUtf8,
    UnicodePropertyNotFound,
    UnicodePropertyValueNotFound,
    EmptyClassNotAllowed,
    Other,
}

impl ErrorKind {
    fn description(&self) -> &str {
        use ErrorKind::*;
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
fn test_empty_class_not_allowed_description() {
    let error = ErrorKind::EmptyClassNotAllowed;
    assert_eq!(error.description(), "empty character classes are not allowed");
}

