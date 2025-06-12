// Answer 0

#[test]
fn test_remove_entry_from_map() {
    use http::header::{HeaderMap, Entry, HOST};
    use http::HeaderValue;

    let mut map = HeaderMap::new();
    map.insert(HOST, HeaderValue::from_static("world"));

    if let Entry::Occupied(e) = map.entry("host") {
        let prev = e.remove();
        assert_eq!("world", prev.to_str().unwrap());
    }

    assert!(!map.contains_key("host"));
}

#[test]
#[should_panic]
fn test_remove_non_existent_entry() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::new();

    // Attempt to remove a non-existent key should panic
    if let Entry::Occupied(e) = map.entry("host") {
        let _ = e.remove(); // This should not be executed
    }
}

