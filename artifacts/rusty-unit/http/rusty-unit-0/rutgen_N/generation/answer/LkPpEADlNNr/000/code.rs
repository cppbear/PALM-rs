// Answer 0

#[test]
fn test_uri_with_valid_string() {
    use http::{Request, Uri};

    let req = Request::builder()
        .uri("https://www.rust-lang.org/")
        .body(())
        .unwrap();

    assert_eq!(req.uri().to_string(), "https://www.rust-lang.org/");
}

#[test]
fn test_uri_with_empty_string() {
    use http::{Request, Uri};

    let req = Request::builder()
        .uri("")
        .body(())
        .unwrap();

    assert_eq!(req.uri().to_string(), "");
}

#[should_panic]
fn test_uri_with_invalid_uri() {
    use http::{Request, Uri};

    let _req = Request::builder()
        .uri("invalid_uri")
        .body(())
        .unwrap();
} 

#[test]
fn test_uri_with_root() {
    use http::{Request, Uri};

    let req = Request::builder()
        .uri("/")
        .body(())
        .unwrap();

    assert_eq!(req.uri().to_string(), "/");
} 

#[test]
fn test_uri_with_unicode() {
    use http::{Request, Uri};

    let req = Request::builder()
        .uri("https://example.com/файл")
        .body(())
        .unwrap();

    assert_eq!(req.uri().to_string(), "https://example.com/файл");
}

