// Answer 0

fn test_rebuild_basic_functionality() {
    struct TestValue;
    let mut map: HeaderMap<TestValue> = HeaderMap::with_capacity(4);
    
    // Insert some entries initially
    map.insert(HeaderName { inner: Repr::Custom }, TestValue);
    map.insert(HeaderName { inner: Repr::Custom }, TestValue);
    map.insert(HeaderName { inner: Repr::Custom }, TestValue);
    map.insert(HeaderName { inner: Repr::Custom }, TestValue);

    // Call the rebuild function
    map.rebuild();

    // Check that all entries are still present
    assert_eq!(map.len(), 4);
}

fn test_rebuild_with_vacant_slots() {
    struct TestValue;
    let mut map: HeaderMap<TestValue> = HeaderMap::with_capacity(8);
    
    // Insert entries with some gaps
    map.insert(HeaderName { inner: Repr::Custom }, TestValue);
    map.insert(HeaderName { inner: Repr::Custom }, TestValue); // let's say this is at index 0
    map.insert(HeaderName { inner: Repr::Custom }, TestValue); // this one will be at index 2

    // Now intentionally leave some indices as vacant (index 1 is vacant)
    
    // Call rebuild
    map.rebuild();

    // All entries should still be retrievable
    assert_eq!(map.len(), 3);
}

#[should_panic]
fn test_rebuild_empty_map() {
    struct TestValue;
    let mut map: HeaderMap<TestValue> = HeaderMap::with_capacity(0);  // Start with a capacity of 0

    // Call rebuild on an empty map (should panic due to no entries)
    map.rebuild();
}

fn test_rebuild_with_edge_case() {
    struct TestValue;
    let mut map: HeaderMap<TestValue> = HeaderMap::with_capacity(8);
    
    // Inserting multiple entries
    for _ in 0..7 {
        map.insert(HeaderName { inner: Repr::Custom }, TestValue);
    }

    // Call rebuild
    map.rebuild();

    // Assert the length of entries from the map
    assert_eq!(map.len(), 7);
}

fn test_rebuild_with_distances() {
    struct TestValue;
    let mut map: HeaderMap<TestValue> = HeaderMap::with_capacity(4);
    
    // Insert entries with specific hashes that cause a distance situation
    map.insert(HeaderName { inner: Repr::Custom }, TestValue);      // Hash 0
    map.insert(HeaderName { inner: Repr::Custom }, TestValue);      // Hash 1
    map.insert(HeaderName { inner: Repr::Custom }, TestValue);      // Hash 2
    map.insert(HeaderName { inner: Repr::Custom }, TestValue);      // Hash 3

    // Call rebuild to check if distance logic works
    map.rebuild();

    // Ensure the map still has the correct number of entries
    assert_eq!(map.len(), 4);
}

