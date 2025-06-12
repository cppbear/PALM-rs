// Answer 0

#[test]
fn test_authority_fmt_empty() {
    let authority = Authority::empty();
    let mut output = Vec::new();
    let result = authority.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), "");
}

#[test]
fn test_authority_fmt_static_str() {
    let static_str = "user:password@host:8080";
    let authority = Authority::from_static(static_str);
    let mut output = Vec::new();
    let result = authority.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), static_str);
}

#[test]
fn test_authority_fmt_from_shared() {
    let bytes = Bytes::from("test@localhost");
    let authority = Authority::from_shared(bytes).expect("Should be valid authority");
    let mut output = Vec::new();
    let result = authority.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), "test@localhost");
}

