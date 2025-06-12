// Answer 0

#[test]
fn test_debug_bytes_ref_empty() {
    let data: &[u8] = b"";
    let bytes_ref = BytesRef(data);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(buffer, r#"b""""#);
}

#[test]
fn test_debug_bytes_ref_printable() {
    let data: &[u8] = b"Hello, World!";
    let bytes_ref = BytesRef(data);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(buffer, r#"b"Hello, World!""#);
}

#[test]
fn test_debug_bytes_ref_with_escapes() {
    let data: &[u8] = b"Line1\nLine2\rLine3\tLine4";
    let bytes_ref = BytesRef(data);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(buffer, r#"b"Line1\nLine2\rLine3\tLine4""#);
}

#[test]
fn test_debug_bytes_ref_with_control_chars() {
    let data: &[u8] = b"Control: \x01\x02\x03\x04\x05";
    let bytes_ref = BytesRef(data);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(buffer, r#"b"Control: \x01\x02\x03\x04\x05""#);
}

#[test]
fn test_debug_bytes_ref_with_quotes_and_backslashes() {
    let data: &[u8] = b"Quote: \" and backslash: \\";
    let bytes_ref = BytesRef(data);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(buffer, r#"b"Quote: \" and backslash: \\""#);
}

#[test]
fn test_debug_bytes_ref_with_zero_byte() {
    let data: &[u8] = b"Null byte: \0";
    let bytes_ref = BytesRef(data);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", bytes_ref);
    assert!(result.is_ok());
    assert_eq!(buffer, r#"b"Null byte: \0""#);
}

