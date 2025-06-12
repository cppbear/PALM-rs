// Answer 0

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_from_static_invalid_uri_no_scheme() {
    let _uri = http::uri::from_static("example.com/foo");
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_from_static_invalid_uri_empty() {
    let _uri = http::uri::from_static("");
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_from_static_invalid_uri_invalid_characters() {
    let _uri = http::uri::from_static("http://example.com/foo#bar@baz");
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_from_static_invalid_uri_missing_host() {
    let _uri = http::uri::from_static("http:///foo");
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_from_static_invalid_uri_too_many_slashes() {
    let _uri = http::uri::from_static("http://example.com///foo");
}

