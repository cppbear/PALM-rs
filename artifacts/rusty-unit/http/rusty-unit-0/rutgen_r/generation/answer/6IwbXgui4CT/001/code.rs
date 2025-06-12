// Answer 0

#[test]
fn test_get_mut_existing_key() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::default();
    map.insert(HOST, "hello".to_string());

    let value = map.get_mut("host").unwrap();
    value.push_str("-world");

    assert_eq!(map.get(HOST).unwrap(), &"hello-world");
}

#[test]
fn test_get_mut_non_existing_key() {
    use http::HeaderMap;

    let mut map = HeaderMap::default();
    
    let result = map.get_mut("non_existing_key");
    assert!(result.is_none());
}

#[test]
#[should_panic]
fn test_get_mut_panic_with_invalid_access() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::default();
    
    // Attempting to access a key that does not exist and expecting a panic.
    let _ = map.get_mut(HOST);
}

