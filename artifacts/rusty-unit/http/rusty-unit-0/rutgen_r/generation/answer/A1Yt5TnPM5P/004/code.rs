// Answer 0

#[test]
fn test_try_with_capacity_zero() {
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(0);
    
    assert!(map.is_ok());
    let header_map = map.unwrap();
    assert!(header_map.is_empty());
    assert_eq!(header_map.capacity(), 0);
}

#[test]
fn test_try_with_capacity_max_size() {
    const MAX_SIZE: usize = 1024; // Assuming a hypothetical max size for testing
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(MAX_SIZE);
    
    assert!(map.is_ok());
    // Ensuring header_map does not exceed MAX_SIZE
    let header_map = map.unwrap();
    assert!(header_map.capacity() <= MAX_SIZE);
}

#[test]
#[should_panic]
fn test_try_with_capacity_too_large() {
    const MAX_SIZE: usize = 1024; // Assuming a hypothetical max size for testing
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(MAX_SIZE + 1);
    
    assert!(map.is_err());
}

