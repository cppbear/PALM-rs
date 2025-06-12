// Answer 0

#[test]
fn test_decimal_empty() {
    let error_kind = ErrorKind::DecimalEmpty;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_kind);
}

#[test]
fn test_decimal_invalid() {
    let error_kind = ErrorKind::DecimalInvalid;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error_kind);
}

