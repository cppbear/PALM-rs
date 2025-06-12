// Answer 0

#[test]
fn test_key_with_occupied_entry() {
    use http::header::{HeaderMap, Entry, HeaderName, HOST};

    let mut map = HeaderMap::new();
    map.insert(HOST, "world".parse().unwrap());

    if let Entry::Occupied(e) = map.entry("host") {
        assert_eq!("host", e.key().as_str());
    }
}

#[test]
#[should_panic]
fn test_key_with_empty_map() {
    use http::header::{HeaderMap, Entry};

    let map = HeaderMap::new();

    // This should panic as there are no entries to occupy
    let _ = map.entry("non-existent");
}

#[test]
fn test_key_with_nonexistent_entry() {
    use http::header::{HeaderMap, Entry};

    let mut map = HeaderMap::new();
    map.insert("test-header", "value".parse().unwrap());

    if let Entry::Vacant(e) = map.entry("non-existent") {
        assert_eq!("non-existent", e.key().as_str());
    }
}

#[test]
fn test_key_with_existing_entry_case_insensitive() {
    use http::header::{HeaderMap, Entry, HeaderName, HOST};

    let mut map = HeaderMap::new();
    map.insert(HOST, "world".parse().unwrap());

    // Case-insensitive lookup
    if let Entry::Occupied(e) = map.entry("Host") {
        assert_eq!("Host", e.key().as_str());
    }
}

