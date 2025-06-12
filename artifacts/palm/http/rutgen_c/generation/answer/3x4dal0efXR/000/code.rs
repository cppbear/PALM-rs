// Answer 0

#[test]
fn test_try_reserve_one_increases_capacity() {
    struct DummyValue;
    let mut header_map: HeaderMap<DummyValue> = HeaderMap::with_capacity(0);
    
    header_map.try_reserve(7).unwrap(); // Ensure we can reserve initially
    assert!(header_map.capacity() > 0); // Check that capacity has increased
}

#[test]
fn test_try_reserve_one_doubles_capacity() {
    struct DummyValue;
    let mut header_map: HeaderMap<DummyValue> = HeaderMap::with_capacity(1);
    
    // Fill the map
    header_map.try_insert("key1", DummyValue).unwrap();
    header_map.try_insert("key2", DummyValue).unwrap();
    header_map.danger.set_yellow(); // Set danger to yellow
    
    // Reserve one, this should trigger capacity doubling
    header_map.try_reserve_one().unwrap();
    assert_eq!(header_map.capacity(), 2); // Check that capacity has doubled
}

#[test]
fn test_try_reserve_one_rebuilds_when_necessary() {
    struct DummyValue;
    let mut header_map: HeaderMap<DummyValue> = HeaderMap::with_capacity(4);
    
    // Fill up the current capacity
    for i in 0..4 {
        header_map.try_insert(format!("key{}", i), DummyValue).unwrap();
    }
    
    // Setting danger to yellow and reserve more
    header_map.danger.set_yellow();
    header_map.try_reserve_one().unwrap(); // This should trigger a rebuild
    
    assert!(header_map.capacity() > 4); // Rebuilding should have increased capacity
}

#[test]
fn test_try_reserve_one_empty_map_initializes_correctly() {
    struct DummyValue;
    let mut header_map: HeaderMap<DummyValue> = HeaderMap::with_capacity(0);
    
    // The initial capacity should be 0
    assert_eq!(header_map.capacity(), 0);
    
    // Trying to reserve one should allocate the initial value
    header_map.try_reserve_one().unwrap();
    
    // After reserving, it should have capacity of 8
    assert_eq!(header_map.capacity(), 8);
}

#[test]
#[should_panic]
fn test_try_reserve_one_exceeding_max_size() {
    struct DummyValue;
    let mut header_map: HeaderMap<DummyValue> = HeaderMap::with_capacity(MAX_SIZE);
    
    // Fill to max size
    for _ in 0..MAX_SIZE {
        header_map.try_insert("key", DummyValue).unwrap();
    }
    
    // This should panic due to reaching max size
    header_map.try_reserve_one().unwrap();
}

