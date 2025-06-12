// Answer 0

#[test]
fn test_with_capacity_zero() {
    let map: HeaderMap<u32> = HeaderMap::with_capacity(0);
    assert!(map.is_empty());
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_with_capacity_minimum() {
    let map: HeaderMap<u32> = HeaderMap::with_capacity(1);
    assert!(map.is_empty());
    assert!(map.capacity() >= 1);
}

#[test]
fn test_with_capacity_standard() {
    let map: HeaderMap<u32> = HeaderMap::with_capacity(10);
    assert!(map.is_empty());
    assert_eq!(map.capacity(), 12); // Assuming the next power of two allocation
}

#[test]
fn test_with_capacity_max_size() {
    let capacity = (1 << 15) - 1; // One less than MAX_SIZE
    let map: HeaderMap<u32> = HeaderMap::with_capacity(capacity);
    assert!(map.is_empty());
    assert!(map.capacity() >= capacity);
}

#[should_panic(expected = "size overflows MAX_SIZE")]
#[test]
fn test_with_capacity_exceed_max_size() {
    let _map: HeaderMap<u32> = HeaderMap::with_capacity(1 << 15); // Exceeds MAX_SIZE
}

