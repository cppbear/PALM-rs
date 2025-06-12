// Answer 0

#[test]
fn test_get_from_occupied_entry() {
    use http::header::{HeaderMap, Entry, HOST};
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello.world".parse().unwrap());

    if let Entry::Occupied(mut e) = map.entry("host") {
        assert_eq!(e.get(), &"hello.world");
    }
}

#[test]
#[should_panic]
fn test_get_from_empty_entry() {
    use http::header::{HeaderMap, Entry, HOST};
    let mut map = HeaderMap::new();

    if let Entry::Occupied(mut e) = map.entry("non-existent") {
        e.get(); // This should panic as there are no values associated with the entry.
    }
}

