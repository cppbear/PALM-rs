// Answer 0

#[test]
fn test_options_with_valid_uri() {
    let request = http::options("https://www.rust-lang.org/")
        .body(())
        .unwrap();
    assert_eq!(*request.method(), http::Method::OPTIONS);
}

#[test]
fn test_options_with_local_uri() {
    let request = http::options("http://localhost:8080/")
        .body(())
        .unwrap();
    assert_eq!(*request.method(), http::Method::OPTIONS);
}

#[test]
fn test_options_with_https_uri() {
    let request = http::options("https://example.com/")
        .body(())
        .unwrap();
    assert_eq!(*request.method(), http::Method::OPTIONS);
}

#[should_panic]
fn test_options_with_bad_uri() {
    let _request: Result<_, _> = http::options("invalid-uri")
        .body(())
        .unwrap();
} 

#[test]
fn test_options_with_an_empty_uri() {
    let request = http::options("")
        .body(())
        .unwrap();
    assert_eq!(*request.method(), http::Method::OPTIONS);
}

