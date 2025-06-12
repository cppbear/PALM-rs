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
fn test_authority_display_maybe_shared() {
    let data: Bytes = Bytes::from("example.org");
    let authority = Authority::from_maybe_shared(data).unwrap();
    let result = format!("{}", authority);
    assert_eq!(result, "example.org");
}

#[test]
fn test_authority_display_maybe_shared_empty() {
    let data: Vec<u8> = vec![];
    let authority = Authority::from_maybe_shared(data).unwrap();
    let result = format!("{}", authority);
    assert_eq!(result, "");
}

