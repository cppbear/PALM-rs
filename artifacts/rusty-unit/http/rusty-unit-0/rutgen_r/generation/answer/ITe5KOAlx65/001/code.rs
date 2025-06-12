// Answer 0

#[test]
fn test_capacity_empty_header_map() {
    let map = HeaderMap::new();
    assert_eq!(0, map.capacity());
}

#[test]
fn test_capacity_after_one_insert() {
    let mut map = HeaderMap::new();
    map.insert(HOST, "hello.world".parse().unwrap());
    assert_eq!(6, map.capacity());
}

#[test]
fn test_capacity_multiple_inserts() {
    let mut map = HeaderMap::new();
    map.insert(HOST, "example.com".parse().unwrap());
    map.insert("User-Agent", "rust-http-client".parse().unwrap());
    map.insert("Accept", "application/json".parse().unwrap());
    assert!(map.capacity() >= 6); // Expecting at least the capacity after three inserts.
}

#[test]
fn test_capacity_large_number_of_inserts() {
    let mut map = HeaderMap::new();
    for i in 0..100 {
        map.insert(format!("Header-{}", i).parse().unwrap(), format!("Value-{}", i).parse().unwrap());
    }
    assert!(map.capacity() > 100); // Ensure capacity increased after many inserts.
}

#[test]
#[should_panic]
fn test_capacity_with_invalid_header() {
    let mut map = HeaderMap::new();
    map.insert("Invalid-Header", "".parse().unwrap()); // Assuming this format should panic or fail in some way.
}

