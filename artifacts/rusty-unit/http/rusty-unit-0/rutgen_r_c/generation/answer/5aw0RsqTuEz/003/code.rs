// Answer 0

#[test]
fn test_try_reserve_exceeding_max_size() {
    struct DummyValue;
    let mut map: HeaderMap<DummyValue> = HeaderMap::with_capacity(1); // Start with a small initial capacity
    let additional = (MAX_SIZE as usize + 1) - map.entries.len(); // Make additional too large to fit

    let result = map.try_reserve(additional);
    assert_eq!(result, Err(MaxSizeReached::new()));
}

#[test]
fn test_try_reserve_when_exceeding_raw_capacity() {
    struct DummyValue;
    
    // Create a map with maximum capacity and fill entries
    let mut map: HeaderMap<DummyValue> = HeaderMap::with_capacity(1 << 14); // Start with a buffer where we can still add one
    map.entries.push(DummyValue);
    map.indices = vec![Pos::none(); 1 << 14].into_boxed_slice(); // Fill indices to simulate full usage

    let additional = 1; // Any additional that would exceed raw capacity
    let result = map.try_reserve(additional);
    assert_eq!(result, Err(MaxSizeReached::new()));
}

#[test]
fn test_try_reserve_boundary_case() {
    struct DummyValue;
    
    // Create a map with entries almost at MAX_SIZE but valid
    let mut map: HeaderMap<DummyValue> = HeaderMap::with_capacity(1 << 14); // Initialize with half capacity
    let additional = (MAX_SIZE as usize) - map.entries.len(); // Exact fit to check boundary

    // Fill entries up to capacity
    for _ in 0..(1 << 14) {
        map.entries.push(DummyValue);
    }
    
    // This should now hit the error due to next power of two exceeding limits
    map.indices = vec![Pos::none(); 1 << 14].into_boxed_slice();

    let result = map.try_reserve(additional);
    assert_eq!(result, Err(MaxSizeReached::new()));
}

