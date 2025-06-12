// Answer 0

#[derive(Debug)]
struct InvalidUri;

#[derive(Debug)]
struct Uri {
    value: String,
}

impl Uri {
    fn try_from(bytes: &[u8]) -> Result<Uri, InvalidUri> {
        let s = String::from_utf8(bytes.to_vec()).map_err(|_| InvalidUri)?;
        if s.is_empty() || !s.starts_with("http://") && !s.starts_with("https://") {
            return Err(InvalidUri);
        }
        Ok(Uri { value: s })
    }
}

#[test]
fn test_uri_from_valid_http() {
    let result = from_str("http://example.com");
    assert!(result.is_ok());
    let uri = result.unwrap();
    assert_eq!(uri.value, "http://example.com");
}

#[test]
fn test_uri_from_valid_https() {
    let result = from_str("https://example.com");
    assert!(result.is_ok());
    let uri = result.unwrap();
    assert_eq!(uri.value, "https://example.com");
}

#[test]
fn test_uri_from_invalid_http() {
    let result = from_str("ftp://example.com");
    assert!(result.is_err());
}

#[test]
fn test_uri_from_empty_string() {
    let result = from_str("");
    assert!(result.is_err());
}

#[test]
fn test_uri_from_invalid_format() {
    let result = from_str("example.com");
    assert!(result.is_err());
}

#[test]
fn test_uri_from_string_with_only_scheme() {
    let result = from_str("http://");
    assert!(result.is_err());
}

