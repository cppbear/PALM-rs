// Answer 0

#[test]
fn test_class_escape_invalid() {
    let error_kind = ErrorKind::ClassEscapeInvalid;
    let mut output = String::new();
    let _ = write!(&mut output, "{}", error_kind);
}

#[test]
fn test_another_class_escape_invalid() {
    let error_kind = ErrorKind::ClassEscapeInvalid;
    let mut output = String::new();
    let _ = write!(&mut output, "{}", error_kind);
}

