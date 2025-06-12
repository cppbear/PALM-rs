// Answer 0

#[test]
fn test_authority_debug_fmt() {
    let authority = Authority::from_static("example.com");
    let mut buffer = String::new();
    let result = authority.fmt(&mut fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
    assert_eq!(buffer, "example.com");
}

#[test]
fn test_authority_debug_fmt_empty() {
    let authority = Authority::empty();
    let mut buffer = String::new();
    let result = authority.fmt(&mut fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
    assert_eq!(buffer, "");
}

