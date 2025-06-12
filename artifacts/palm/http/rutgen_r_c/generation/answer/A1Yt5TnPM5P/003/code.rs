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
fn test_try_with_capacity_valid() {
    let capacity = 12; 
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(capacity);
    assert!(map.is_ok());
    let header_map = map.unwrap();
    assert!(header_map.is_empty());
    assert!(header_map.capacity() >= capacity);
}

#[test]
#[should_panic]
fn test_try_with_capacity_exceeds_max_size() {
    let max_size_capacity = (MAX_SIZE as usize) + 1; 
    let _map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(max_size_capacity);
}

#[test]
fn test_try_with_capacity_boundary() {
    let boundary_capacity = (MAX_SIZE as usize); 
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(boundary_capacity);
    assert!(map.is_ok());
    let header_map = map.unwrap();
    assert!(header_map.capacity() == boundary_capacity);
}

