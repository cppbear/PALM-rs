// Answer 0

#[test]
fn test_parse_full_http() {
    let input = Bytes::from_static(b"http://example.com/path/to/resource");
    let _ = parse_full(input);
}

#[test]
fn test_parse_full_https() {
    let input = Bytes::from_static(b"https://example.com/path/to/resource");
    let _ = parse_full(input);
}

#[test]
fn test_parse_full_http_with_query() {
    let input = Bytes::from_static(b"http://example.com/path?query=123");
    let _ = parse_full(input);
}

#[test]
fn test_parse_full_https_with_fragment() {
    let input = Bytes::from_static(b"https://example.com/path#fragment");
    let _ = parse_full(input);
}

#[test]
fn test_parse_full_http_min_length() {
    let input = Bytes::from_static(b"http://x.com/");
    let _ = parse_full(input);
}

#[test]
fn test_parse_full_https_min_length() {
    let input = Bytes::from_static(b"https://x.com/");
    let _ = parse_full(input);
}

#[test]
fn test_parse_full_http_max_length() {
    let long_uri = b"http://" + b"example.com/path".repeat(4095 - 18).as_slice(); // 65534 total length
    let input = Bytes::from_static(long_uri);
    let _ = parse_full(input);
}

#[test]
fn test_parse_full_https_max_length() {
    let long_uri = b"https://" + b"example.com/path".repeat(4095 - 19).as_slice();
    let input = Bytes::from_static(long_uri);
    let _ = parse_full(input);
}

#[test]
fn test_parse_full_http_with_authority_only() {
    let input = Bytes::from_static(b"http://example.com");
    let _ = parse_full(input);
}

#[test]
fn test_parse_full_http_with_empty_path() {
    let input = Bytes::from_static(b"http://example.com/");
    let _ = parse_full(input);
}

