// Answer 0

#[test]
fn test_from_shared_empty() {
    let input = Bytes::from_static(b"");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_single_slash() {
    let input = Bytes::from_static(b"/");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_single_star() {
    let input = Bytes::from_static(b"*");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_single_invalid() {
    let input = Bytes::from_static(b"a");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_max_len() {
    let input = Bytes::from(vec![b'a'; 65535]);  // Length equals MAX_LEN
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_valid_path() {
    let input = Bytes::from_static(b"/path/to/resource");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_invalid_uri_char() {
    let input = Bytes::from_static(b"invalid\xFFchar");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_invalid_query() {
    let input = Bytes::from_static(b"invalid?query&param");
    let _ = Uri::from_shared(input);
}

