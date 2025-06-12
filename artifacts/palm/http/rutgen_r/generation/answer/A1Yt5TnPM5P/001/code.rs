// Answer 0

#[test]
fn test_try_with_capacity_zero() {
    // Since capacity == 0 is false, we should test some non-zero capacity.
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(0);
    assert!(map.is_ok());
    let header_map = map.unwrap();
    assert_eq!(header_map.mask, 0);
    assert_eq!(header_map.indices.len(), 0);
    assert!(header_map.entries.is_empty());
    assert!(header_map.extra_values.is_empty());
    assert_eq!(header_map.danger, Danger::Green);
}

#[test]
fn test_try_with_capacity_non_zero() {
    // Testing with a regular capacity that does not exceed max size.
    let capacity = 10;
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(capacity);
    assert!(map.is_ok());
    let header_map = map.unwrap();
    
    // Check the mask and other properties of HeaderMap.
    assert!(header_map.mask > 0);
    assert!(header_map.indices.len() > 0);
    assert!(header_map.entries.capacity() > 0);
    assert!(header_map.extra_values.is_empty());
    assert_eq!(header_map.danger, Danger::Green);
}

#[test]
fn test_try_with_capacity_exceeds_max_size() {
    // Attempt to create a HeaderMap with a capacity that exceeds the max size.
    let capacity = usize::MAX; // Using the maximum usize to cause an error.
    let map: Result<HeaderMap<u32>, MaxSizeReached> = HeaderMap::try_with_capacity(capacity);
    assert!(map.is_err());
}

