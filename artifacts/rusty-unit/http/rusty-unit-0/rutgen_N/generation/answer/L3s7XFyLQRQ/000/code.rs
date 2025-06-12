// Answer 0

#[test]
fn test_remove_entry_existing() {
    use http::header::{HeaderMap, HeaderName, Entry, HeaderValue, HOST};

    let mut map = HeaderMap::new();
    map.insert(HOST, "world".parse().unwrap());

    if let Entry::Occupied(e) = map.entry("host") {
        let (key, value) = e.remove_entry();
        assert_eq!("host", key.as_str());
        assert_eq!("world", value.to_str().unwrap());
    }

    assert!(!map.contains_key("host"));
}

#[test]
#[should_panic]
fn test_remove_entry_non_existing() {
    use http::header::{HeaderMap, HeaderName, Entry, HOST};

    let mut map = HeaderMap::new();

    if let Entry::Occupied(e) = map.entry("host") {
        e.remove_entry(); // This should panic as "host" does not exist
    }
}

