// Answer 0

#[test]
fn test_error_syntax_max_length() {
    let err = Error::Syntax("a".repeat(79));
    let mut buf = Vec::new();
    let _ = write!(&mut buf, "{:?}", err);
}

#[test]
fn test_error_syntax_short_message() {
    let err = Error::Syntax("short message".to_string());
    let mut buf = Vec::new();
    let _ = write!(&mut buf, "{:?}", err);
}

#[test]
fn test_error_syntax_empty_message() {
    let err = Error::Syntax("".to_string());
    let mut buf = Vec::new();
    let _ = write!(&mut buf, "{:?}", err);
}

#[test]
fn test_error_syntax_exceed_length() {
    let err = Error::Syntax("a".repeat(80));
    let mut buf = Vec::new();
    let _ = write!(&mut buf, "{:?}", err);
}

#[test]
fn test_error_syntax_special_characters() {
    let err = Error::Syntax("!@#$%^&*()_+-=<>?".to_string());
    let mut buf = Vec::new();
    let _ = write!(&mut buf, "{:?}", err);
}

