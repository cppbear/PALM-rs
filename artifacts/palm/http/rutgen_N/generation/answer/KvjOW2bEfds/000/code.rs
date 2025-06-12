// Answer 0

#[test]
fn test_parse_full_valid_standard_scheme() {
    use bytes::Bytes;
    use crate::uri::{parse_full, Uri, InvalidUri, Scheme2, Authority, PathAndQuery};

    let input = Bytes::from("http://example.com/path?query=value");
    let result: Result<Uri, InvalidUri> = parse_full(input);

    assert!(result.is_ok());
    let uri = result.unwrap();
    assert_eq!(uri.scheme, Some(Scheme2::Standard("http".into())));
    assert_eq!(uri.authority.data.to_string(), "example.com");
    assert_eq!(uri.path_and_query.to_string(), "/path?query=value");
}

#[test]
fn test_parse_full_valid_other_scheme() {
    use bytes::Bytes;
    use crate::uri::{parse_full, Uri, InvalidUri, Scheme2, Authority, PathAndQuery};

    let input = Bytes::from("ftp://example.com/path?query=value");
    let result: Result<Uri, InvalidUri> = parse_full(input);

    assert!(result.is_ok());
    let uri = result.unwrap();
    assert_eq!(uri.scheme, Some(Scheme2::Other(Box::new(ByteStr::from_utf8_unchecked("ftp".as_bytes().to_vec())))));
    assert_eq!(uri.authority.data.to_string(), "example.com");
    assert_eq!(uri.path_and_query.to_string(), "/path?query=value");
}

#[test]
fn test_parse_full_missing_authority() {
    use bytes::Bytes;
    use crate::uri::{parse_full, InvalidUri};

    let input = Bytes::from("http:///path");
    let result: Result<Uri, InvalidUri> = parse_full(input);

    assert!(result.is_err());
}

#[test]
fn test_parse_full_invalid_format() {
    use bytes::Bytes;
    use crate::uri::{parse_full, InvalidUri};

    let input = Bytes::from("invalid_uri_without_scheme");
    let result: Result<Uri, InvalidUri> = parse_full(input);

    assert!(result.is_err());
}

#[test]
fn test_parse_full_absolute_no_scheme() {
    use bytes::Bytes;
    use crate::uri::{parse_full, InvalidUri};

    let input = Bytes::from("example.com/path");
    let result: Result<Uri, InvalidUri> = parse_full(input);

    assert!(result.is_err());
}

