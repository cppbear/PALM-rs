// Answer 0

#[test]
fn test_header_map_values() {
    use http::HeaderMap;
    use http::header::{CONTENT_LENGTH, HOST};
    use std::collections::hash_map::Iter;

    struct Values<'a, T> {
        inner: Iter<'a, T, T>,
    }

    let mut map = HeaderMap::new();

    map.insert(HOST, "hello".parse().unwrap());
    map.append(HOST, "goodbye".parse().unwrap());
    map.insert(CONTENT_LENGTH, "123".parse().unwrap());

    let values: Vec<_> = map.values().collect();

    assert_eq!(values.len(), 3);
    assert!(values.contains(&"hello".parse().unwrap()));
    assert!(values.contains(&"goodbye".parse().unwrap()));
    assert!(values.contains(&"123".parse().unwrap()));
}

#[test]
fn test_empty_header_map_values() {
    use http::HeaderMap;

    let map = HeaderMap::new();
    let values: Vec<_> = map.values().collect();

    assert!(values.is_empty());
}

