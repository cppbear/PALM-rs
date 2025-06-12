// Answer 0

#[test]
fn test_from_shared_valid_case_with_query() {
    let src = Bytes::from_static(b"/example?query=value");
    let result = PathAndQuery::from_shared(src);
    assert!(result.is_ok());
    let pq = result.unwrap();
    assert_eq!(pq.query, 8);
}

#[test]
fn test_from_shared_invalid_uri_character() {
    let src = Bytes::from_static(b"/example\xFF");
    let result = PathAndQuery::from_shared(src);
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::InvalidUriChar);
    }
}

#[test]
fn test_from_shared_fragment_present() {
    let src = Bytes::from_static(b"/example#fragment");
    let result = PathAndQuery::from_shared(src);
    assert!(result.is_ok());
    let pq = result.unwrap();
    assert_eq!(pq.data.bytes, Bytes::from_static(b"/example"));
    assert_eq!(pq.query, u16::MAX);
}

#[test]
fn test_from_shared_empty() {
    let src = Bytes::from_static(b"");
    let result = PathAndQuery::from_shared(src);
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::Empty);
    }
}

#[test]
fn test_from_shared_invalid_utf8() {
    let src = Bytes::from_static(b"/example\xC3\x28"); // Invalid UTF-8 byte sequence
    let result = PathAndQuery::from_shared(src);
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::InvalidUriChar);
    }
}

#[test]
fn test_from_shared_with_only_fragment() {
    let src = Bytes::from_static(b"#fragment");
    let result = PathAndQuery::from_shared(src);
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::PathAndQueryMissing);
    }
}

#[test]
fn test_from_shared_with_invalid_character() {
    let src = Bytes::from_static(b"/example|invalid"); // '|' is invalid in URI paths
    let result = PathAndQuery::from_shared(src);
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::InvalidUriChar);
    }
}

