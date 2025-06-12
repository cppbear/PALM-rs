// Answer 0

#[test]
fn test_capacity_empty_header_map() {
    let map: HeaderMap = HeaderMap::with_capacity(0);
    assert_eq!(0, map.capacity());
}

#[test]
fn test_capacity_after_insertion() {
    let mut map: HeaderMap = HeaderMap::with_capacity(10);
    // Assuming that insertion causes growth to at least a certain threshold
    map.insert("Host", "example.com".parse().unwrap());
    assert_eq!(usable_capacity(10), map.capacity());
}

#[test]
fn test_capacity_large_capacity() {
    let mut map: HeaderMap = HeaderMap::with_capacity(1000);
    assert_eq!(usable_capacity(1000), map.capacity());
    for i in 0..15 {
        map.insert(format!("Header-{}", i), format!("Value-{}", i).parse().unwrap());
    }
    assert_eq!(usable_capacity(1000), map.capacity());
}

#[test]
fn test_capacity_reallocation() {
    let mut map: HeaderMap = HeaderMap::with_capacity(1);
    // On first insert, it should grow
    map.insert("Initial", "value".parse().unwrap());
    assert!(map.capacity() > 1);
}

#[test]
fn test_capacity_with_multiple_insertions() {
    let mut map: HeaderMap = HeaderMap::with_capacity(20);
    for i in 0..30 {
        map.insert(format!("Header-{}", i), format!("Value-{}", i).parse().unwrap());
    }
    // Check capacity after multiple insertions
    assert!(map.capacity() > 20);
}

