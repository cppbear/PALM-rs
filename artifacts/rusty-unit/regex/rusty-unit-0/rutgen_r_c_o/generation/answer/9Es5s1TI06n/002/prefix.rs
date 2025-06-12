// Answer 0

#[test]
fn test_unsupported_look_around() {
    use regex_syntax::ErrorKind;

    let error_kind = ErrorKind::UnsupportedLookAround;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_kind);
}

#[test]
fn test_unsupported_look_around_display() {
    use regex_syntax::ErrorKind;

    let error_kind = ErrorKind::UnsupportedLookAround;
    let display_string = error_kind.to_string();
}

