// Answer 0

#[test]
fn test_head_with_valid_uri() {
    use http::Request;
    use std::convert::TryInto;

    let request = Request::head("https://www.rust-lang.org/")
        .body(())
        .unwrap();
    assert_eq!(request.method(), "HEAD");
    assert_eq!(request.uri().to_string(), "https://www.rust-lang.org/");
}

#[test]
#[should_panic]
fn test_head_with_invalid_uri() {
    use http::Request;
    use std::convert::TryInto;

    let _request = Request::head("")
        .body(())
        .unwrap();
}

#[test]
fn test_head_with_another_valid_uri() {
    use http::Request;
    use std::convert::TryInto;

    let request = Request::head("https://example.com")
        .body(())
        .unwrap();
    assert_eq!(request.method(), "HEAD");
    assert_eq!(request.uri().to_string(), "https://example.com");
}

#[test]
fn test_head_with_uri_containing_query() {
    use http::Request;
    use std::convert::TryInto;

    let request = Request::head("https://www.example.com?query=param")
        .body(())
        .unwrap();
    assert_eq!(request.method(), "HEAD");
    assert_eq!(request.uri().to_string(), "https://www.example.com?query=param");
}

#[test]
fn test_head_with_uri_containing_fragment() {
    use http::Request;
    use std::convert::TryInto;

    let request = Request::head("https://www.example.com#section")
        .body(())
        .unwrap();
    assert_eq!(request.method(), "HEAD");
    assert_eq!(request.uri().to_string(), "https://www.example.com#section");
}

