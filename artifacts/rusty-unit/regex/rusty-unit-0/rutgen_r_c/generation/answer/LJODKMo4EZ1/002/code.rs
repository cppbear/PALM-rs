// Answer 0

#[test]
fn test_description_empty_class_not_allowed() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ErrorKind {
        UnicodeNotAllowed,
        InvalidUtf8,
        UnicodePropertyNotFound,
        UnicodePropertyValueNotFound,
        EmptyClassNotAllowed,
        #[doc(hidden)]
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

    let error = ErrorKind::EmptyClassNotAllowed;
    assert_eq!(error.description(), "empty character classes are not allowed");
}

