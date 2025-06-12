// Answer 0

#[test]
fn test_put_with_valid_uri() {
    let request = http::Request::put("https://www.rust-lang.org/")
        .body(())
        .unwrap();
    assert_eq!(request.method(), http::Method::PUT);
    assert_eq!(request.uri().to_string(), "https://www.rust-lang.org/");
}

#[test]
#[should_panic]
fn test_put_with_invalid_uri() {
    struct InvalidUri;

    impl TryInto<http::Uri> for InvalidUri {
        type Error = http::Error;

        fn try_into(self) -> Result<http::Uri, Self::Error> {
            Err(http::Error::new("Invalid URI"))
        }
    }

    let _request = http::Request::put(InvalidUri).body(()).unwrap();
}

#[test]
fn test_put_with_empty_uri() {
    let request = http::Request::put("")
        .body(())
        .unwrap();
    assert_eq!(request.method(), http::Method::PUT);
    assert_eq!(request.uri().to_string(), "");
}

#[test]
fn test_put_with_partial_uri() {
    let request = http::Request::put("https://")
        .body(())
        .unwrap();
    assert_eq!(request.method(), http::Method::PUT);
    assert_eq!(request.uri().to_string(), "https://");
}

