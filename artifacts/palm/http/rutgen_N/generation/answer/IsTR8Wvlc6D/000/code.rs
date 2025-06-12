// Answer 0

#[test]
fn test_get_without_values() {
    use http::HeaderMap;
    use http::header::HOST;

    let map = HeaderMap::new();
    assert!(map.get("host").is_none());
}

#[test]
fn test_get_with_single_value() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    assert_eq!(map.get(HOST).unwrap(), &"hello");
    assert_eq!(map.get("host").unwrap(), &"hello");
}

#[test]
fn test_get_with_multiple_values() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    map.insert(HOST, "hello".parse().unwrap());
    map.append(HOST, "world".parse().unwrap());
    assert_eq!(map.get("host").unwrap(), &"hello");
}

