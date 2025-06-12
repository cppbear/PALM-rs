// Answer 0

#[test]
fn test_try_with_capacity_zero() {
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(0);
    assert!(map.is_ok());
    let map = map.unwrap();
    assert!(map.is_empty());
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_try_with_capacity_small_positive() {
    let capacity = 1;
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(capacity);
    assert!(map.is_ok());
    let map = map.unwrap();
    assert!(map.is_empty());
    assert_eq!(map.capacity(), 2); // As to_raw_capacity(1) would return 2
}

#[test]
fn test_try_with_capacity_boundary_max_size() {
    // Test for maximum raw capacity that does not exceed MAX_SIZE
    let capacity = (MAX_SIZE >> 1) + (MAX_SIZE >> 2); // raw_cap should equal MAX_SIZE
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(capacity);
    assert!(map.is_err()); // Expecting an error because raw_cap would exceed MAX_SIZE
}

#[test]
#[should_panic(expected = "requested capacity too large")]
fn test_try_with_capacity_overflow() {
    let capacity = usize::MAX; // Trigger overflow in to_raw_capacity
    let _map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(capacity);
} 

#[test]
fn test_try_with_capacity_large_value() {
    let capacity = 12; // A larger number still within bounds
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(capacity);
    assert!(map.is_ok());
    let map = map.unwrap();
    assert!(map.is_empty());
    assert_eq!(map.capacity(), 16); // As to_raw_capacity(12) would return 16
}

