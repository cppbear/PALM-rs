// Answer 0

#[test]
fn test_header_map_new_is_empty() {
    let map = http::HeaderMap::new();
    assert!(map.is_empty());
}

#[test]
fn test_header_map_new_capacity() {
    let map = http::HeaderMap::new();
    assert_eq!(0, map.capacity());
}

