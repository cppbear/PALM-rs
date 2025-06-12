// Answer 0

#[test]
fn test_from_static_valid_uri() {
    let uri = Uri::from_static("http://example.com/foo");
    assert_eq!(uri.host().unwrap(), "example.com");
    assert_eq!(uri.path(), "/foo");
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_from_static_empty_uri() {
    Uri::from_static("");
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_from_static_too_long_uri() {
    let long_uri = "http://" + &("a".repeat(usize::MAX - 1)); // Assuming the string is too long
    Uri::from_static(&long_uri);
}

#[test]
fn test_from_static_root_path() {
    let uri = Uri::from_static("/");
    assert!(uri.authority().is_none());
    assert_eq!(uri.path(), "/");
}

#[test]
fn test_from_static_star_path() {
    let uri = Uri::from_static("*");
    assert!(uri.authority().is_none());
    assert_eq!(uri.path(), "*");
}

#[test]
fn test_from_static_invalid_uri() {
    let invalid_uri = "http://"; // Missing host
    assert!(std::panic::catch_unwind(|| {
        Uri::from_static(invalid_uri);
    }).is_err());
}

