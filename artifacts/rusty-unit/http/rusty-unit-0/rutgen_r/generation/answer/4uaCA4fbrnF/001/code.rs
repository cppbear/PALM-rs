// Answer 0

#[test]
fn test_into_key_vacant_entry() {
    use http::header::{HeaderMap, Entry};

    let mut map = HeaderMap::new();

    if let Entry::Vacant(v) = map.entry("x-hello") {
        let key = v.into_key();
        assert_eq!(key.as_str(), "x-hello");
    } else {
        panic!("Expected a Vacant entry");
    }
}

#[test]
fn test_into_key_multiple_vacant_entries() {
    use http::header::{HeaderMap, Entry};

    let mut map = HeaderMap::new();
    
    let key1 = "x-hello";
    let key2 = "x-world";

    if let Entry::Vacant(v1) = map.entry(key1) {
        let key_name1 = v1.into_key();
        assert_eq!(key_name1.as_str(), key1);
    } else {
        panic!("Expected a Vacant entry for x-hello");
    }

    if let Entry::Vacant(v2) = map.entry(key2) {
        let key_name2 = v2.into_key();
        assert_eq!(key_name2.as_str(), key2);
    } else {
        panic!("Expected a Vacant entry for x-world");
    }
}

#[test]
fn test_into_key_empty_key() {
    use http::header::{HeaderMap, Entry};

    let mut map = HeaderMap::new();
    
    if let Entry::Vacant(v) = map.entry("") {
        let key = v.into_key();
        assert_eq!(key.as_str(), "");
    } else {
        panic!("Expected a Vacant entry for an empty key");
    }
}

