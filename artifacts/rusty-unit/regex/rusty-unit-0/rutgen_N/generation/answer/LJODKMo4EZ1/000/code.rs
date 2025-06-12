// Answer 0

#[test]
fn test_unicode_not_allowed_description() {
    struct UnicodeNotAllowed;

    impl UnicodeNotAllowed {
        fn description(&self) -> &str {
            match *self {
                UnicodeNotAllowed => "Unicode not allowed here",
                _ => unreachable!(),
            }
        }
    }

    let error = UnicodeNotAllowed;
    assert_eq!(error.description(), "Unicode not allowed here");
}

#[test]
fn test_invalid_utf8_description() {
    struct InvalidUtf8;

    impl InvalidUtf8 {
        fn description(&self) -> &str {
            match *self {
                InvalidUtf8 => "pattern can match invalid UTF-8",
                _ => unreachable!(),
            }
        }
    }

    let error = InvalidUtf8;
    assert_eq!(error.description(), "pattern can match invalid UTF-8");
}

#[test]
fn test_unicode_property_not_found_description() {
    struct UnicodePropertyNotFound;

    impl UnicodePropertyNotFound {
        fn description(&self) -> &str {
            match *self {
                UnicodePropertyNotFound => "Unicode property not found",
                _ => unreachable!(),
            }
        }
    }

    let error = UnicodePropertyNotFound;
    assert_eq!(error.description(), "Unicode property not found");
}

#[test]
fn test_unicode_property_value_not_found_description() {
    struct UnicodePropertyValueNotFound;

    impl UnicodePropertyValueNotFound {
        fn description(&self) -> &str {
            match *self {
                UnicodePropertyValueNotFound => "Unicode property value not found",
                _ => unreachable!(),
            }
        }
    }

    let error = UnicodePropertyValueNotFound;
    assert_eq!(error.description(), "Unicode property value not found");
}

#[test]
fn test_empty_class_not_allowed_description() {
    struct EmptyClassNotAllowed;

    impl EmptyClassNotAllowed {
        fn description(&self) -> &str {
            match *self {
                EmptyClassNotAllowed => "empty character classes are not allowed",
                _ => unreachable!(),
            }
        }
    }

    let error = EmptyClassNotAllowed;
    assert_eq!(error.description(), "empty character classes are not allowed");
}

