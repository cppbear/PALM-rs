// Answer 0

#[test]
fn test_append() {
    use http::HeaderMap;
    use http::header::HOST;
    
    // Create a HeaderMap with default initial capacity
    let mut map = HeaderMap::with_capacity(10);
    
    // Test appending a value to an empty map
    assert!(map.append(HOST, "world".parse().unwrap()));
    assert_eq!(map.len(), 1);
    
    // Test appending another value to the same key
    assert!(!map.append(HOST, "earth".parse().unwrap()));
    assert_eq!(map.len(), 1);
    
    // Retrieve values associated with the key
    let values = map.get_all(HOST);
    let mut i = values.iter();
    assert_eq!("world", *i.next().unwrap());
    assert_eq!("earth", *i.next().unwrap());
    
    // Check the append method returns false if the key already exists
    assert!(!map.append(HOST, "universe".parse().unwrap()));
    let values = map.get_all(HOST);
    let mut i = values.iter();
    assert_eq!("world", *i.next().unwrap());
    assert_eq!("earth", *i.next().unwrap());

    // Test panics when exceeding max capacity
    let mut big_map = HeaderMap::with_capacity(1); // Small capacity for testing panic
    big_map.append(HOST, "one".parse().unwrap());
    let result = std::panic::catch_unwind(|| {
        big_map.append(HOST, "two".parse().unwrap());
    });
    assert!(result.is_err());
}

