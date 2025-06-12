// Answer 0

#[test]
fn test_scheme_with_http_uri() {
    use http::uri::{Scheme, Uri};

    let uri: Uri = "http://example.org/hello/world".parse().unwrap();
    assert_eq!(uri.scheme(), Some(&Scheme::HTTP));
}

#[test]
fn test_scheme_with_https_uri() {
    use http::uri::{Scheme, Uri};

    let uri: Uri = "https://example.org/secure/data".parse().unwrap();
    assert_eq!(uri.scheme(), Some(&Scheme::HTTPS));
}

#[test]
fn test_scheme_with_ftp_uri() {
    use http::uri::{Scheme, Uri};

    let uri: Uri = "ftp://example.com/file.txt".parse().unwrap();
    assert_eq!(uri.scheme(), Some(&Scheme::FTP));
}

#[test]
fn test_scheme_with_custom_scheme_uri() {
    use http::uri::{Scheme, Uri};

    let uri: Uri = "customscheme://resource/path".parse().unwrap();
    assert_eq!(uri.scheme(), Some(&Scheme::CUSTOMSCHEME));
}

