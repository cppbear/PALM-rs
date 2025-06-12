// Answer 0

#[test]
fn test_uri_with_valid_string() {
    let req = Request::builder()
        .uri("https://www.rust-lang.org/")
        .body(())
        .unwrap();
    assert_eq!(req.head.uri, "https://www.rust-lang.org/");
}

#[test]
fn test_uri_with_empty_string() {
    let req = Request::builder()
        .uri("")
        .body(())
        .unwrap();
    assert_eq!(req.head.uri, "");
}

#[test]
#[should_panic]
fn test_uri_with_invalid_uri() {
    let req = Request::builder()
        .uri("invalid_uri")
        .body(())
        .unwrap();
    // The above line should panic if "invalid_uri" is a malformed URI
}

#[test]
fn test_uri_with_persistent_change() {
    let req = Request::builder()
        .uri("https://example.com/")
        .body(())
        .unwrap();
    assert_eq!(req.head.uri, "https://example.com/");
    
    let req_changed = Request::builder()
        .uri("https://rust-lang.org/")
        .body(())
        .unwrap();
    assert_eq!(req_changed.head.uri, "https://rust-lang.org/");
}

#[test]
fn test_uri_with_partial_uri() {
    let req = Request::builder()
        .uri("https://rust-lang.org/path")
        .body(())
        .unwrap();
    assert_eq!(req.head.uri, "https://rust-lang.org/path");
}

