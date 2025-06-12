// Answer 0

#[test]
fn test_remove_existing_key() {
    use http::header::HOST;
    use http::HeaderValue;
    let mut map = HeaderMap::with_capacity(10);
    map.insert(HOST, HeaderValue::from_static("hello.world"));

    let prev = map.remove(HOST).unwrap();
    assert_eq!(prev, HeaderValue::from_static("hello.world"));
    assert!(map.is_empty());
}

#[test]
fn test_remove_non_existing_key() {
    use http::header::HOST;
    use http::HeaderValue;
    let mut map = HeaderMap::with_capacity(10);
    
    assert!(map.remove(HOST).is_none());
}

#[test]
fn test_remove_multiple_entries() {
    use http::header::HOST;
    use http::HeaderValue;
    let mut map = HeaderMap::with_capacity(10);
    map.append(HOST, HeaderValue::from_static("first"));
    map.append(HOST, HeaderValue::from_static("second"));

    let prev = map.remove(HOST).unwrap();
    assert_eq!(prev, HeaderValue::from_static("first"));
    assert_eq!(map.len(), 1);
    
    let prev_second = map.remove(HOST).unwrap();
    assert_eq!(prev_second, HeaderValue::from_static("second"));
    assert!(map.is_empty());
}

#[test]
fn test_remove_key_and_check_if_empty() {
    use http::header::HOST;
    use http::HeaderValue;
    let mut map = HeaderMap::with_capacity(10);
    map.insert(HOST, HeaderValue::from_static("hello.world"));

    assert!(!map.is_empty());
    map.remove(HOST);
    assert!(map.is_empty());
}

