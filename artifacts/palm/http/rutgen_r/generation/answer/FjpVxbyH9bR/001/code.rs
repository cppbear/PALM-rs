// Answer 0

#[test]
fn test_remove_existing_entry() {
    use http::header::{HeaderMap, HeaderName, HeaderValue};
    use http::header::HOST;
    
    let mut map = HeaderMap::new();
    map.insert(HOST, HeaderValue::from_static("world"));
    
    // Attempt to remove an existing entry
    if let http::header::Entry::Occupied(e) = map.entry("host") {
        let prev = e.remove();
        assert_eq!(prev, HeaderValue::from_static("world"));
    }
    
    // Ensure the key no longer exists
    assert!(!map.contains_key("host"));
}

#[test]
#[should_panic(expected = "entry for key `nonexistent` does not exist")]
fn test_remove_non_existing_entry() {
    use http::header::{HeaderMap, HeaderName, HeaderValue};
    
    let mut map = HeaderMap::new();
    
    // Attempt to remove a non-existing entry should panic
    if let http::header::Entry::Occupied(_) = map.entry("nonexistent") {
        panic!("entry for key `nonexistent` does not exist");
    }
}

#[test]
fn test_remove_multiple_entries() {
    use http::header::{HeaderMap, HeaderName, HeaderValue};
    use http::header::CONTENT_TYPE;
    
    let mut map = HeaderMap::new();
    map.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    map.insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    
    // Attempt to remove the CONTENT_TYPE entry
    if let http::header::Entry::Occupied(e) = map.entry("content-type") {
        let prev = e.remove();
        assert_eq!(prev, HeaderValue::from_static("text/plain")); // Only the first value returned
    }
    
    // Ensure the key no longer exists
    assert!(!map.contains_key("content-type"));
}

