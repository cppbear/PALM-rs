// Answer 0

#[test]
fn test_post_with_valid_uri() {
    let request = post("https://www.rust-lang.org/")
        .body(())
        .unwrap();
    assert_eq!(request.method(), Method::POST);
    assert_eq!(request.uri().to_string(), "https://www.rust-lang.org/");
}

#[test]
fn test_post_with_http_scheme() {
    let request = post("http://example.com/")
        .body(())
        .unwrap();
    assert_eq!(request.method(), Method::POST);
    assert_eq!(request.uri().to_string(), "http://example.com/");
}

#[test]
fn test_post_with_max_length_uri() {
    let long_uri = "https://" + &"a".repeat(2048) + ".com/";
    let request = post(long_uri)
        .body(())
        .unwrap();
    assert_eq!(request.method(), Method::POST);
}

#[should_panic]
fn test_post_with_invalid_uri() {
    let _request = post("invalid_uri$%^")
        .body(())
        .unwrap();
}

#[should_panic]
fn test_post_with_empty_uri() {
    let _request = post("")
        .body(())
        .unwrap();
}

