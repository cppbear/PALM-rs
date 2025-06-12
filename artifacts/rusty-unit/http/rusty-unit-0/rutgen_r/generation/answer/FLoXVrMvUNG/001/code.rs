// Answer 0

#[test]
fn test_patch_with_valid_uri() {
    use http::*;
    let request = Request::patch("https://www.rust-lang.org/")
        .body(())
        .unwrap();

    assert_eq!(request.method(), Method::PATCH);
    assert_eq!(request.uri().to_string(), "https://www.rust-lang.org/");
}

#[test]
#[should_panic]
fn test_patch_with_invalid_uri() {
    use http::*;
    let request = Request::patch("invalid_uri")
        .body(())
        .unwrap();
}

#[test]
fn test_patch_with_empty_uri() {
    use http::*;
    let request = Request::patch("")
        .body(())
        .unwrap();

    assert_eq!(request.method(), Method::PATCH);
    assert_eq!(request.uri().to_string(), "");
}

#[test]
fn test_patch_with_special_characters_uri() {
    use http::*;
    let request = Request::patch("https://example.com/path/?query=test&param=1#fragment")
        .body(())
        .unwrap();

    assert_eq!(request.method(), Method::PATCH);
    assert_eq!(request.uri().to_string(), "https://example.com/path/?query=test&param=1#fragment");
}

