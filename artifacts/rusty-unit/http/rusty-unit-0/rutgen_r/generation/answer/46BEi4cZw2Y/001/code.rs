// Answer 0

#[test]
fn test_get_mut_with_existing_entry() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();
    map.insert(HOST, "hello.world".to_string());

    if let Entry::Occupied(mut e) = map.entry("host") {
        e.get_mut().push_str("-2");
        assert_eq!(e.get(), &"hello.world-2");
    }
}

#[should_panic]
fn test_get_mut_with_non_existent_entry() {
    use http::header::{HeaderMap};

    let mut map = HeaderMap::default();

    if let Entry::Occupied(mut e) = map.entry("nonexistent") {
        // This should panic since there are no values associated with the entry
        e.get_mut();
    }
} 

#[test]
fn test_get_mut_with_multiple_insertions() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();
    map.insert(HOST, "first.value".to_string());
    map.insert(HOST, "second.value".to_string());

    if let Entry::Occupied(mut e) = map.entry("host") {
        e.get_mut().push_str("-modified");
        assert_eq!(e.get(), &"second.value-modified");
    }
} 

#[should_panic]
fn test_get_mut_without_any_insertions() {
    use http::header::{HeaderMap};

    let mut map = HeaderMap::default();

    if let Entry::Occupied(mut e) = map.entry("empty") {
        // This should panic since there are no values associated with the entry
        e.get_mut();
    }
}

