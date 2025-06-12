// Answer 0

#[test]
fn test_header_map_new_empty() {
    let map = HeaderMap::new();
    assert!(map.is_empty());
    assert_eq!(0, map.capacity());
}

#[test]
fn test_header_map_new_capacity_zero() {
    let map = HeaderMap::<HeaderValue>::try_with_capacity(0).unwrap();
    assert!(map.is_empty());
    assert_eq!(0, map.capacity());
}

#[test]
#[should_panic]
fn test_header_map_try_with_capacity_max_size() {
    let capacity = (1u64 << 16) as usize; // Just over the MAX_SIZE
    let _ = HeaderMap::<HeaderValue>::try_with_capacity(capacity).unwrap();
}

#[test]
fn test_header_map_try_with_capacity_valid() {
    let map = HeaderMap::<HeaderValue>::try_with_capacity(1).unwrap();
    assert!(map.is_empty());
    assert!(map.capacity() >= 1);
} 

#[test]
fn test_header_map_new_non_zero_capacity() {
    // Since HeaderMap::new() cannot be tested for non-zero parameters,
    // we will ensure that a valid call does not panic.
    let map = HeaderMap::<HeaderValue>::try_with_capacity(1).unwrap();
    assert!(map.is_empty()); 
    assert!(map.capacity() >= 1); 
}

