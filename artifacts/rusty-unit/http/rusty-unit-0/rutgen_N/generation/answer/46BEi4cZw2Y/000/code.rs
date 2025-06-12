// Answer 0

#[test]
fn test_get_mut_on_occupied_entry() {
    use http::header::{HeaderMap, HeaderName, HEADER_VALUE};

    let mut map = HeaderMap::default();
    let key = HeaderName::from_static("host");
    map.insert(key.clone(), HEADER_VALUE::from_static("hello.world"));

    if let Entry::Occupied(mut entry) = map.entry(key) {
        entry.get_mut().push_str("-2");
        assert_eq!(entry.get(), "hello.world-2");
    } else {
        panic!("Expected entry to be occupied");
    }
}

#[test]
#[should_panic]
fn test_get_mut_on_empty_entry() {
    use http::header::{HeaderMap, HeaderName};

    let mut map = HeaderMap::default();
    let key = HeaderName::from_static("host");

    if let Entry::Vacant(entry) = map.entry(key) {
        entry.get_mut(); // This should panic
    }
}

