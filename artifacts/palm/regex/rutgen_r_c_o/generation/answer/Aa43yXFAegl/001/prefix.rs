// Answer 0

#[test]
fn test_fmt_unicode_not_allowed() {
    let error_kind = ErrorKind::UnicodeNotAllowed;
    let mut formatter = String::new();
    let _ = write!(&mut formatter, "{}", error_kind);
}

#[test]
fn test_fmt_invalid_utf8() {
    let error_kind = ErrorKind::InvalidUtf8;
    let mut formatter = String::new();
    let _ = write!(&mut formatter, "{}", error_kind);
}

#[test]
fn test_fmt_unicode_property_not_found() {
    let error_kind = ErrorKind::UnicodePropertyNotFound;
    let mut formatter = String::new();
    let _ = write!(&mut formatter, "{}", error_kind);
}

#[test]
fn test_fmt_unicode_property_value_not_found() {
    let error_kind = ErrorKind::UnicodePropertyValueNotFound;
    let mut formatter = String::new();
    let _ = write!(&mut formatter, "{}", error_kind);
}

#[test]
fn test_fmt_empty_class_not_allowed() {
    let error_kind = ErrorKind::EmptyClassNotAllowed;
    let mut formatter = String::new();
    let _ = write!(&mut formatter, "{}", error_kind);
}

#[test]
#[should_panic]
fn test_fmt_nonexhaustive() {
    let error_kind = ErrorKind::__Nonexhaustive;
    let mut formatter = String::new();
    let _ = write!(&mut formatter, "{}", error_kind);
}

