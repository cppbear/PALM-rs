// Answer 0

#[test]
fn test_insert_mult_with_multiple_values() {
    use http::header::{HeaderMap, Entry, HOST};
    use std::str::FromStr;

    let mut map = HeaderMap::new();
    map.insert(HOST, "world".parse().unwrap());
    map.append(HOST, "world2".parse().unwrap());

    if let Entry::Occupied(mut e) = map.entry("host") {
        let mut prev = e.insert_mult("earth".parse().unwrap());
        assert_eq!("world", prev.next().unwrap());
        assert_eq!("world2", prev.next().unwrap());
        assert!(prev.next().is_none());
    }

    assert_eq!("earth", map["host"]);
}

#[test]
fn test_insert_mult_on_empty_entry() {
    use http::header::{HeaderMap, Entry, HOST};
    use std::str::FromStr;

    let mut map = HeaderMap::new();

    if let Entry::Vacant(e) = map.entry("host") {
        let mut prev = e.insert_mult("earth".parse().unwrap());
        assert!(prev.next().is_none());
    }

    assert_eq!("earth", map["host"]);
}

#[test]
fn test_insert_mult_overwriting_value() {
    use http::header::{HeaderMap, Entry, HOST};
    use std::str::FromStr;

    let mut map = HeaderMap::new();
    map.insert(HOST, "original".parse().unwrap());

    if let Entry::Occupied(mut e) = map.entry("host") {
        let mut prev = e.insert_mult("new_value".parse().unwrap());
        assert_eq!("original", prev.next().unwrap());
        assert!(prev.next().is_none());
    }

    assert_eq!("new_value", map["host"]);
}

