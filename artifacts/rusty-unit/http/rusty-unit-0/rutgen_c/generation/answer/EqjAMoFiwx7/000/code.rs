// Answer 0

#[test]
fn test_from_static_valid() {
    let authority = Authority::from_static("example.com");
    assert_eq!(authority.host(), "example.com");
}

#[test]
#[should_panic]
fn test_from_static_empty() {
    Authority::from_static("");
}

#[test]
#[should_panic]
fn test_from_static_invalid_chars() {
    Authority::from_static("ex@ample.com");
}

