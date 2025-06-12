// Answer 0

#[test]
fn test_insert_entry_occupied() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::new();
    map.insert(HOST, "hello.world".parse().unwrap());

    if let Entry::Occupied(mut e) = map.entry("host") {
        let prev = e.insert("earth".parse().unwrap());
        assert_eq!("hello.world", prev);
        assert_eq!("earth", map["host"]);
    } else {
        panic!("Expected Entry::Occupied but got Entry::Vacant");
    }
}

#[test]
fn test_insert_entry_empty() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::new();

    if let Entry::Vacant(e) = map.entry("host") {
        e.insert("new.host".parse().unwrap());
        assert_eq!("new.host", map["host"]);
    } else {
        panic!("Expected Entry::Vacant but got Entry::Occupied");
    }
}

#[test]
fn test_insert_entry_overwrite() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::new();
    map.insert(HOST, "initial.host".parse().unwrap());

    if let Entry::Occupied(mut e) = map.entry("host") {
        let prev = e.insert("overwritten.host".parse().unwrap());
        assert_eq!("initial.host", prev);
        assert_eq!("overwritten.host", map["host"]);
    } else {
        panic!("Expected Entry::Occupied but got Entry::Vacant");
    }
}

#[test]
fn test_insert_entry_with_invalid_value() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::new();
    map.insert(HOST, "valid.host".parse().unwrap());

    if let Entry::Occupied(mut e) = map.entry("host") {
        // Attempting to insert an invalid header value to test panic conditions.
        assert!(std::panic::catch_unwind(|| {
            e.insert("invalid value with space".parse::<String>().unwrap())
        }).is_err());
    } else {
        panic!("Expected Entry::Occupied but got Entry::Vacant");
    }
}

