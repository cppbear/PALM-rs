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
    
    let result = map.get_mut("non-existing-key");
    
    assert!(result.is_none());
}

#[test]
fn test_get_mut_multiple_inserts() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::default();
    map.insert(HOST, "first".to_string());
    map.insert(HOST, "second".to_string());

    let value = map.get_mut("host").unwrap();
    value.push_str("-modified");

    assert_eq!(map.get(HOST).unwrap(), &"first-modified");
}

#[test]
fn test_get_mut_empty_map() {
    use http::HeaderMap;

    let mut map = HeaderMap::default();
    
    let result = map.get_mut("empty-key");
    
    assert!(result.is_none());
}

