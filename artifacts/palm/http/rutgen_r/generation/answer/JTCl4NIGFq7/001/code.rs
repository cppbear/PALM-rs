// Answer 0

#[test]
fn test_insert_mult_with_existing_values() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::new();
    map.insert(HOST, "first".parse().unwrap());
    map.append(HOST, "second".parse().unwrap());

    if let Entry::Occupied(mut entry) = map.entry("host") {
        let mut previous = entry.insert_mult("new_value".parse().unwrap());
        assert_eq!("first", previous.next().unwrap());
        assert_eq!("second", previous.next().unwrap());
        assert!(previous.next().is_none());
    }

    assert_eq!("new_value", map["host"]);
}

#[test]
fn test_insert_mult_with_empty_map() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::new();

    if let Entry::Vacant(entry) = map.entry("host") {
        let mut previous = entry.insert_mult("new_value".parse().unwrap());
        assert!(previous.next().is_none());
    }
    
    assert_eq!("new_value", map["host"]);
}

#[test]
fn test_insert_mult_with_non_string_values() {
    use http::header::{HeaderMap, Entry, USER_AGENT};
    
    let mut map = HeaderMap::new();
    map.insert(USER_AGENT, "Mozilla/5.0".parse().unwrap());

    if let Entry::Occupied(mut entry) = map.entry("user-agent") {
        let mut previous = entry.insert_mult("CustomAgent/1.0".parse().unwrap());
        assert_eq!("Mozilla/5.0", previous.next().unwrap());
        assert!(previous.next().is_none());
    }

    assert_eq!("CustomAgent/1.0", map["user-agent"]);
}

#[test]
#[should_panic]
fn test_insert_mult_should_panic_on_invalid_key() {
    use http::header::{HeaderMap, Entry};

    let mut map = HeaderMap::new();

    if let Entry::Occupied(_entry) = map.entry("invalid_key") {
        // This should panic because the entry does not exist.
        _entry.insert_mult("value".parse().unwrap());
    }
}

