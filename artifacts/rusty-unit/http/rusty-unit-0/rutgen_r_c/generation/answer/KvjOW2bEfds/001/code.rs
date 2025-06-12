// Answer 0

#[test]
fn test_parse_full_http() {
    let input = Bytes::from_static(b"http://example.com/path?query=1");
    let result = parse_full(input);

    match result {
        Ok(uri) => {
            assert_eq!(uri.scheme, Scheme2::Standard(Protocol::Http));
            assert_eq!(uri.authority.as_str(), "example.com");
            assert_eq!(uri.path_and_query.query, 10); // '?query=1' ends at index 10
            assert_eq!(uri.path_and_query.data.bytes, Bytes::from_static(b"/path"));
        },
        Err(_) => panic!("Expected valid URI but got an error"),
    }
}

#[test]
fn test_parse_full_https() {
    let input = Bytes::from_static(b"https://example.com/path?query=2");
    let result = parse_full(input);

    match result {
        Ok(uri) => {
            assert_eq!(uri.scheme, Scheme2::Standard(Protocol::Https));
            assert_eq!(uri.authority.as_str(), "example.com");
            assert_eq!(uri.path_and_query.query, 10); // '?query=2' ends at index 10
            assert_eq!(uri.path_and_query.data.bytes, Bytes::from_static(b"/path"));
        },
        Err(_) => panic!("Expected valid URI but got an error"),
    }
}

#[test]
fn test_parse_full_unsupported_scheme() {
    let input = Bytes::from_static(b"ftp://example.com");
    let result = parse_full(input);

    match result {
        Ok(_) => panic!("Expected an error for unsupported scheme"),
        Err(e) => {
            assert_eq!(format!("{:?}", e), "InvalidFormat");
        }
    }
}

#[test]
fn test_parse_full_no_authority() {
    let input = Bytes::from_static(b"http://");
    let result = parse_full(input);

    match result {
        Ok(_) => panic!("Expected an error for missing authority"),
        Err(e) => {
            assert_eq!(format!("{:?}", e), "InvalidFormat");
        }
    }
}

#[test]
fn test_parse_full_without_scheme() {
    let input = Bytes::from_static(b"example.com/path");
    let result = parse_full(input);

    match result {
        Ok(uri) => {
            assert_eq!(uri.scheme, Scheme2::None);
            assert_eq!(uri.authority.as_str(), "example.com");
            assert_eq!(uri.path_and_query.data.bytes, Bytes::from_static(b"/path"));
        },
        Err(_) => panic!("Expected valid URI but got an error"),
    }
}

