// Answer 0

#[test]
fn test_insert_entry_occupied() {
    use http::header::{HeaderMap, Entry, HOST};    
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello.world".parse().unwrap());

    if let Entry::Occupied(mut e) = map.entry("host") {
        let prev = e.insert("earth".parse().unwrap());
        assert_eq!("hello.world", prev);
    }

    assert_eq!("earth", map["host"]);
}

#[test]
fn test_insert_new_entry() {
    use http::header::{HeaderMap, HeaderName, HeaderValue};
    
    struct DummyHeader {
        name: HeaderName,
        value: HeaderValue,
    }

    let mut map = HeaderMap::new();
    let dummy_header = DummyHeader {
        name: "dummy-header".parse().unwrap(),
        value: "initial-value".parse().unwrap(),
    };
    
    map.insert(dummy_header.name.clone(), dummy_header.value.clone());

    if let Entry::Occupied(mut entry) = map.entry("dummy-header") {
        let prev = entry.insert("new-value".parse().unwrap());
        assert_eq!("initial-value", prev);
    }

    assert_eq!("new-value", map["dummy-header"]);
}

#[test]
#[should_panic]
fn test_insert_entry_non_existing() {
    use http::header::{HeaderMap, Entry};

    let mut map = HeaderMap::new();
    
    if let Entry::Occupied(_) = map.entry("non-existing-header") {
        panic!("Expected an unoccupied entry.");
    } else {
        // Simulate a panic scenario
        panic!("This entry doesn't exist yet.");
    }
}

