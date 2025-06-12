// Answer 0

#[test]
fn test_insert() {
    use http::header::{HeaderMap, HeaderValue};
    
    let mut map = HeaderMap::with_capacity(1);
    let key = HeaderValue::from_static("host");
    map.insert(key.clone(), HeaderValue::from_static("hello.world"));

    if let Entry::Occupied(mut entry) = map.entry(key) {
        let previous_value = entry.insert(HeaderValue::from_static("earth"));
        assert_eq!(previous_value, HeaderValue::from_static("hello.world"));
    } else {
        panic!("Entry should be occupied");
    }

    assert_eq!(map.get(key).unwrap(), &HeaderValue::from_static("earth"));
}

#[test]
fn test_insert_empty_map() {
    use http::header::{HeaderMap, HeaderValue};
    
    let mut map = HeaderMap::new();
    let key = HeaderValue::from_static("content-type");

    if let Entry::Occupied(mut entry) = map.entry(key.clone()) {
        let previous_value = entry.insert(HeaderValue::from_static("application/json"));
        assert_eq!(previous_value, HeaderValue::from_static("application/json"));
    } else {
        panic!("Entry should be occupied after insert");
    }

    assert_eq!(map.get(key).unwrap(), &HeaderValue::from_static("application/json"));
}

#[test]
#[should_panic]
fn test_insert_on_non_existent_entry() {
    use http::header::{HeaderMap, HeaderValue};

    let mut map = HeaderMap::new();
    let key = HeaderValue::from_static("header-not-present");
    
    // This should panic because the entry is not occupied
    if let Entry::Occupied(mut entry) = map.entry(key) {
        entry.insert(HeaderValue::from_static("value"));
    } else {
        panic!("Expected an occupied entry");
    }
}

