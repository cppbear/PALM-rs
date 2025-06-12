// Answer 0

#[test]
fn test_append_new_key() {
    use http::HeaderMap;
    use http::HeaderValue;
    use http::header::HOST;

    let mut map = HeaderMap::with_capacity(10);
    let result = map.append(HOST, "world".parse::<HeaderValue>().unwrap());
    assert!(result);
    assert_eq!(map.len(), 1);
    assert!(!map.is_empty());
}

#[test]
fn test_append_existing_key() {
    use http::HeaderMap;
    use http::HeaderValue;
    use http::header::HOST;

    let mut map = HeaderMap::with_capacity(10);
    map.append(HOST, "world".parse::<HeaderValue>().unwrap());
    let result = map.append(HOST, "earth".parse::<HeaderValue>().unwrap());
    assert!(!result);
    assert_eq!(map.len(), 1);
}

#[test]
#[should_panic]
fn test_append_exceed_max_capacity() {
    use http::HeaderMap;
    use http::HeaderValue;

    let mut map = HeaderMap::with_capacity(1); // Set a small capacity
    map.append("key1", "value1".parse::<HeaderValue>().unwrap());
    map.append("key2", "value2".parse::<HeaderValue>().unwrap()); // This should panic
}

