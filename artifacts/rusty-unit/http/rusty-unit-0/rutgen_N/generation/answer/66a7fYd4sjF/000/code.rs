// Answer 0

#[test]
fn test_key_returns_correct_key_for_occupied_entry() {
    use http::header::{HeaderMap, HeaderName, Entry, HOST};

    let mut map = HeaderMap::new();
    map.insert(HOST, "world".parse().unwrap());

    if let Entry::Occupied(e) = map.entry("host") {
        assert_eq!(HeaderName::from_static("host"), e.key());
    } else {
        panic!("Expected an occupied entry for the key \"host\"");
    }
}

#[test]
fn test_key_for_non_existent_entry() {
    use http::header::{HeaderMap, Entry};

    let map = HeaderMap::new();

    if let Entry::Vacant(_) = map.entry("non-existent") {
        // Expected behavior since the entry is vacant.
    } else {
        panic!("Expected a vacant entry for the key \"non-existent\"");
    }
}

