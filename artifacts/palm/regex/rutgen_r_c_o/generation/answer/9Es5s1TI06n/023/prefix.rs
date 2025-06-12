// Answer 0

#[test]
fn test_escape_hex_empty() {
    let error_kind = ErrorKind::EscapeHexEmpty;
    let mut buffer = String::new();
    let _ = error_kind.fmt(&mut buffer);
}

#[test]
fn test_escape_hex_invalid() {
    let error_kind = ErrorKind::EscapeHexInvalid;
    let mut buffer = String::new();
    let _ = error_kind.fmt(&mut buffer);
}

#[test]
fn test_escape_hex_invalid_digit() {
    let error_kind = ErrorKind::EscapeHexInvalidDigit;
    let mut buffer = String::new();
    let _ = error_kind.fmt(&mut buffer);
}

#[test]
fn test_escape_unrecognized() {
    let error_kind = ErrorKind::EscapeUnrecognized;
    let mut buffer = String::new();
    let _ = error_kind.fmt(&mut buffer);
}

