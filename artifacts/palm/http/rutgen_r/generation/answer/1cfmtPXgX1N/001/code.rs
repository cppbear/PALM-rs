// Answer 0

#[test]
fn test_into_mut_with_multiple_values() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();
    map.insert(HOST, "example.com".to_string());
    map.append(HOST, "example.org".to_string());

    if let Entry::Occupied(e) = map.entry(HOST) {
        let value = e.into_mut(); // should not panic
        value.push_str("-modified");
    }

    assert_eq!("example.com-modified", map[HOST]);
}

#[test]
#[should_panic]
fn test_into_mut_with_no_values() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();

    // Attempt to access an entry that does not exist, which should cause a panic
    if let Entry::Occupied(e) = map.entry(HOST) {
        e.into_mut(); // This should panic
    }
}

#[test]
fn test_into_mut_with_single_value() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();
    map.insert(HOST, "example.net".to_string());

    if let Entry::Occupied(e) = map.entry(HOST) {
        let value = e.into_mut(); // should not panic
        value.push_str("-changed");
    }

    assert_eq!("example.net-changed", map[HOST]);
}

