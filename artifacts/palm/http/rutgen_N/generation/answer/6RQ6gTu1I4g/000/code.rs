// Answer 0

#[test]
fn test_contains_key_empty_map() {
    use http::HeaderMap;
    use http::header::HOST;

    let map = HeaderMap::new();
    assert!(!map.contains_key(HOST));
}

#[test]
fn test_contains_key_with_inserted_value() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    map.insert(HOST, "world".parse().unwrap());
    assert!(map.contains_key(HOST));
}

#[test]
fn test_contains_key_case_insensitivity() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    map.insert(HOST, "localhost".parse().unwrap());
    assert!(map.contains_key("host"));
    assert!(map.contains_key("HOST"));
    assert!(map.contains_key("Host"));
}

#[test]
fn test_contains_key_non_existent_key() {
    use http::HeaderMap;
    use http::header::HOST;
    use http::header::USER_AGENT;

    let mut map = HeaderMap::new();
    map.insert(HOST, "world".parse().unwrap());
    assert!(!map.contains_key(USER_AGENT));
}

