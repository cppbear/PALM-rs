// Answer 0

#[test]
fn test_remove_existing_key() {
    use http::HeaderMap;
    use http::header::HOST;
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello.world".parse().unwrap());
    
    let prev = map.remove(HOST).unwrap();
    assert_eq!("hello.world", prev);
}

#[test]
fn test_remove_non_existing_key() {
    use http::HeaderMap;
    use http::header::HOST;
    
    let mut map = HeaderMap::new();
    assert!(map.remove(HOST).is_none());
}

#[test]
fn test_remove_multiple_values() {
    use http::HeaderMap;
    use http::header::HOST;
    
    let mut map = HeaderMap::new();
    map.insert(HOST, "first.value".parse().unwrap());
    map.insert(HOST, "second.value".parse().unwrap());
    
    let first_prev = map.remove(HOST).unwrap();
    assert_eq!("first.value", first_prev);
    
    let second_prev = map.remove(HOST).unwrap();
    assert_eq!("second.value", second_prev);
    
    assert!(map.remove(HOST).is_none());
}

