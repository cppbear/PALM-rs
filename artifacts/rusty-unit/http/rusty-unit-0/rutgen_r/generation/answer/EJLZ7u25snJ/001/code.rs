// Answer 0

#[test]
fn test_try_insert_new_key() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    assert!(map.try_insert(HOST, "world".parse().unwrap()).unwrap().is_none());
    assert!(!map.is_empty());
}

#[test]
fn test_try_insert_existing_key() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    assert!(map.try_insert(HOST, "world".parse().unwrap()).unwrap().is_none());
    
    let prev = map.try_insert(HOST, "earth".parse().unwrap()).unwrap().unwrap();
    assert_eq!("world", prev);
}

#[test]
fn test_try_insert_exceed_capacity() {
    use http::HeaderMap;
    use http::header::{HOST, USER_AGENT};
    
    let mut map = HeaderMap::new();
    
    // Assuming a max size of 2 for example. This will depend on actual implementation limits.
    assert!(map.try_insert(HOST, "value1".parse().unwrap()).is_ok());
    assert!(map.try_insert(USER_AGENT, "value2".parse().unwrap()).is_ok());

    // This should cause an error due to max capacity
    // Replace "user-defined limit reached error" with the actual error type you expect in your implementation
    let err = map.try_insert(HOST, "value3".parse().unwrap()).unwrap_err();
    assert!(matches!(err, MaxSizeReached));
}

