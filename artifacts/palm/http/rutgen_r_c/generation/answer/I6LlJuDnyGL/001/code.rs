// Answer 0

#[test]
#[should_panic]
fn test_from_static_empty_string() {
    Uri::from_static("");
}

#[test]
#[should_panic]
fn test_from_static_too_long_string() {
    let long_uri = "http://example.com/".repeat(1000); // Exceeds MAX_LEN defined
    Uri::from_static(long_uri.as_str());
}

#[test]
#[should_panic]
fn test_from_static_single_invalid_char() {
    Uri::from_static("!");
}

#[test]
#[should_panic]
fn test_from_static_invalid_domain() {
    Uri::from_static("http://-invalid-domain");
}

#[test]
#[should_panic]
fn test_from_static_blank_host() {
    Uri::from_static("http://:80/");
}

#[test]
fn test_from_static_valid_uri() {
    let uri = Uri::from_static("http://example.com/foo");
    assert_eq!(uri.host().unwrap(), "example.com");
    assert_eq!(uri.path(), "/foo");
}

