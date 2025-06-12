// Answer 0

#[test]
fn test_authority_display_empty() {
    let authority = Authority::empty();
    let result = format!("{}", authority);
    assert_eq!(result, "");
}

#[test]
fn test_authority_display_static() {
    let authority = Authority::from_static("example.com");
    let result = format!("{}", authority);
    assert_eq!(result, "example.com");
}

#[test]
fn test_authority_display_shared() {
    let bytes = Bytes::from_static(b"example.org");
    let authority = Authority::from_shared(bytes).expect("valid authority");
    let result = format!("{}", authority);
    assert_eq!(result, "example.org");
}

#[test]
fn test_authority_display_maybe_shared() {
    let bytes = b"example.net";
    let authority = Authority::from_maybe_shared(bytes).expect("valid authority");
    let result = format!("{}", authority);
    assert_eq!(result, "example.net");
}

