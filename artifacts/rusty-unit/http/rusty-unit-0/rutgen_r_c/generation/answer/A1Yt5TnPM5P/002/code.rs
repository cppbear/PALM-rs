// Answer 0

#[test]
fn test_try_with_capacity_zero_capacity() {
    let result: Result<http::HeaderMap<u32>, http::MaxSizeReached> = http::HeaderMap::try_with_capacity(0);
    assert!(result.is_ok());
    let map = result.unwrap();
    assert!(map.is_empty());
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_try_with_capacity_exceed_max_size() {
    // Set a capacity that results in an overflow when calling to_raw_capacity
    let capacity = std::usize::MAX; // This should trigger the overflow condition in to_raw_capacity
    let result: Result<http::HeaderMap<u32>, http::MaxSizeReached> = http::HeaderMap::try_with_capacity(capacity);
    assert!(result.is_err());
} 

#[test]
fn test_try_with_capacity_exceeding_raw_capacity() {
    // Use a large but valid capacity to force the raw_capacity check
    let capacity = 131072; // This should exceed the defined MAX_SIZE which is 32768 (1 << 15)
    let result: Result<http::HeaderMap<u32>, http::MaxSizeReached> = http::HeaderMap::try_with_capacity(capacity);
    assert!(result.is_err());
} 

#[test]
fn test_try_with_capacity_valid_capacity() {
    let capacity = 10; // This is a valid capacity
    let result: Result<http::HeaderMap<u32>, http::MaxSizeReached> = http::HeaderMap::try_with_capacity(capacity);
    assert!(result.is_ok());
    let map = result.unwrap();
    assert!(map.is_empty());
    assert!(map.capacity() > 10); // Ensure capacity is more than requested
}

