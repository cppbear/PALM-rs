// Answer 0

#[test]
fn test_scheme_absolute_http() {
    use http::uri::{Scheme, Uri};

    let uri: Uri = "http://example.org/hello/world".parse().unwrap();
    assert_eq!(uri.scheme(), Some(&Scheme::HTTP));
}

#[test]
fn test_scheme_absolute_https() {
    use http::uri::{Scheme, Uri};

    let uri: Uri = "https://example.org/hello/world".parse().unwrap();
    assert_eq!(uri.scheme(), Some(&Scheme::HTTPS));
}

#[test]
fn test_scheme_relative() {
    use http::Uri;

    let uri: Uri = "/hello/world".parse().unwrap();
    assert!(uri.scheme().is_none());
}

#[test]
fn test_scheme_empty() {
    use http::Uri;

    let uri: Uri = "".parse().unwrap();
    assert!(uri.scheme().is_none());
}

#[test]
fn test_scheme_with_port() {
    use http::uri::{Scheme, Uri};

    let uri: Uri = "http://example.com:8080/path".parse().unwrap();
    assert_eq!(uri.scheme(), Some(&Scheme::HTTP));
}

