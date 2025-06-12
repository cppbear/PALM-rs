// Answer 0

#[test]
fn test_into_mut_multiple_values() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();
    map.insert(HOST, "hello.world".to_string());
    map.append(HOST, "hello.earth".to_string());

    if let Entry::Occupied(e) = map.entry("host") {
        e.into_mut().push_str("-2");
    }

    assert_eq!("hello.world-2", map["host"]);
}

#[test]
#[should_panic(expected = "no values associated with the entry")]
fn test_into_mut_no_values() {
    use http::header::{HeaderMap, Entry, HOST};

    let mut map = HeaderMap::default();

    if let Entry::Occupied(e) = map.entry("host") {
        e.into_mut(); // This should panic since there are no values.
    }
}

