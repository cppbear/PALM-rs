// Answer 0

#[test]
fn test_fmt_with_empty_authority() {
    let authority = Authority::empty();
    let _ = format!("{:?}", authority);
}

#[test]
fn test_fmt_with_valid_authority() {
    let authority = Authority::from_static("example.com");
    let _ = format!("{:?}", authority);
}

#[test]
fn test_fmt_with_valid_authority_max_length() {
    let long_str = "a".repeat(1024);
    let authority = Authority::from_static(&long_str);
    let _ = format!("{:?}", authority);
}

#[test]
#[should_panic]
fn test_fmt_with_invalid_utf8_authority() {
    let invalid_utf8: &[u8] = &[0xFF, 0xFE, 0xFD]; // Example invalid UTF-8 bytes
    let authority = Authority::from_maybe_shared(invalid_utf8).unwrap();
    let _ = format!("{:?}", authority);
}

