// Answer 0

#[test]
fn test_try_with_capacity_zero() {
    let map: HeaderMap<u32> = HeaderMap::try_with_capacity(0).unwrap();
    assert!(map.is_empty());
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_try_with_capacity_positive() {
    let capacity = 10;
    let map_result = HeaderMap::<u32>::try_with_capacity(capacity);
    assert!(map_result.is_ok());
    let map = map_result.unwrap();
    assert!(map.is_empty());
    assert!(map.capacity() >= 10); // The actual capacity may be greater due to internal sizing
}

#[test]
#[should_panic]
fn test_try_with_capacity_too_large() {
    // Assuming MAX_SIZE is defined as 32768 (1 << 15), we trigger overflow
    let _ = HeaderMap::<u32>::try_with_capacity(usize::MAX);
}

#[test]
fn test_try_with_capacity_boundary() {
    // Test the boundary of valid and invalid capacity
    let result = HeaderMap::<u32>::try_with_capacity(1 << 15);
    assert!(result.is_ok());

    let result_over = HeaderMap::<u32>::try_with_capacity(1 << 15 + 1);
    assert!(result_over.is_err());
}

