// Answer 0

#[test]
fn test_insert_new_key() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    assert!(map.insert(HOST, "world".parse().unwrap()).is_none());
    assert!(!map.is_empty());
}

#[test]
fn test_insert_existing_key() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    assert!(map.insert(HOST, "world".parse().unwrap()).is_none());
    
    let prev = map.insert(HOST, "earth".parse().unwrap()).unwrap();
    assert_eq!("world", prev);
    assert_eq!(map.get(HOST).unwrap(), "earth");
}

#[should_panic(expected = "size overflows MAX_SIZE")]
#[test]
fn test_insert_exceeds_max_capacity() {
    use http::HeaderMap;
    use http::header::HeaderName;

    let mut map = HeaderMap::new();
    for _ in 0..=HeaderMap::max_size() { // Assuming max_size method exists
        let _ = map.insert(HeaderName::from_static("x-test"), "value".parse().unwrap());
    }
}

