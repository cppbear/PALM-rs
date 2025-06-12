// Answer 0

#[test]
fn test_remove_entry_existing() {
    use http::header::{HeaderMap, Entry, HeaderName};

    let mut map = HeaderMap::new();
    let key = HeaderName::from_static("host");
    map.insert(key.clone(), "world".parse().unwrap());

    if let Entry::Occupied(e) = map.entry("host") {
        let (returned_key, returned_value) = e.remove_entry();
        assert_eq!(key, returned_key);
        assert_eq!("world", returned_value);
    }

    assert!(!map.contains_key("host"));
}

#[test]
#[should_panic]
fn test_remove_entry_non_existent() {
    use http::header::{HeaderMap, Entry, HeaderName};

    let mut map = HeaderMap::new();
    let key = HeaderName::from_static("host");

    if let Entry::Occupied(e) = map.entry("host") {
        let (returned_key, returned_value) = e.remove_entry();
        assert_eq!(key, returned_key);
        assert_eq!("world", returned_value);
    }
}

#[test]
fn test_remove_entry_multiple_values() {
    use http::header::{HeaderMap, Entry, HeaderName};

    let mut map = HeaderMap::new();
    let key1 = HeaderName::from_static("host");
    let key2 = HeaderName::from_static("host");
    map.insert(key1.clone(), "world".parse().unwrap());
    map.insert(key2.clone(), "earth".parse().unwrap());

    if let Entry::Occupied(e) = map.entry("host") {
        let (returned_key, returned_value) = e.remove_entry();
        assert_eq!(key1, returned_key);
        assert_eq!("world", returned_value);
    }

    assert!(map.get("host").is_some());
    if let Entry::Occupied(e) = map.entry("host") {
        let (returned_key, returned_value) = e.remove_entry();
        assert_eq!(key2, returned_key);
        assert_eq!("earth", returned_value);
    }

    assert!(!map.contains_key("host"));
}

