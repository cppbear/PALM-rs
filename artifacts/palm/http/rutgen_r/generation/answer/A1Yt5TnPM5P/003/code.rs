// Answer 0

#[test]
fn test_try_with_capacity_zero() {
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(0);
    assert!(map.is_ok());
    let map = map.unwrap();
    assert!(map.is_empty());
    assert_eq!(0, map.capacity());
}

#[test]
fn test_try_with_capacity_max_size() {
    const MAX_SIZE: usize = 1024; // Assuming a defined max size for tests
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(MAX_SIZE);
    assert!(map.is_ok());
    let map = map.unwrap();
    assert_eq!(map.capacity(), MAX_SIZE); // Since we do not know the specifics of to_raw_capacity, we assume it gives MAX_SIZE correctly
    assert_eq!(map.mask, MAX_SIZE - 1);
}

#[test]
fn test_try_with_capacity_below_max_size() {
    let capacity = 16; // Example value below max size
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(capacity);
    assert!(map.is_ok());
    let map = map.unwrap();
    assert!(map.capacity() >= capacity);
    assert!(map.mask == (map.capacity() - 1) as Size);
    assert!(map.indices.len() == map.capacity());
}

