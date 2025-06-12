// Answer 0

#[test]
fn test_clear_map() {
    use http::HeaderMap;
    use http::header::HOST;

    let mut map = HeaderMap::new();
    map.insert(HOST, "hello.world".parse().unwrap());

    // Ensure the map is not empty before clearing
    assert!(!map.is_empty());

    // Clear the map
    map.clear();

    // After clearing, the map should be empty
    assert!(map.is_empty());
    
    // Ensure the capacity remains greater than zero
    assert!(map.capacity() > 0);
}

#[test]
fn test_clear_map_with_multiple_entries() {
    use http::HeaderMap;
    use http::header::{HOST, USER_AGENT};

    let mut map = HeaderMap::new();
    map.insert(HOST, "hello.world".parse().unwrap());
    map.insert(USER_AGENT, "test-agent".parse().unwrap());

    // Ensure the map is not empty before clearing
    assert!(!map.is_empty());

    // Clear the map
    map.clear();

    // After clearing, the map should be empty
    assert!(map.is_empty());

    // Ensure the capacity remains greater than zero
    assert!(map.capacity() > 0);
}

#[test]
fn test_clear_empty_map() {
    use http::HeaderMap;

    let mut map = HeaderMap::new();

    // Ensure the map is empty before attempting to clear
    assert!(map.is_empty());

    // Clear the map
    map.clear();

    // After clearing, the map should still be empty
    assert!(map.is_empty());

    // Ensure the capacity remains greater than zero
    assert!(map.capacity() > 0);
}

