// Answer 0

#[test]
fn test_try_insert_entry_max_size_reached() {
    const MAX_SIZE: usize = 1 << 15; // Using the same MAX_SIZE defined in the context

    struct TestValue;

    // Initialize a HeaderMap with a capacity equal to MAX_SIZE
    let mut map: HeaderMap<TestValue> = HeaderMap::with_capacity(MAX_SIZE);

    // Fill the map to its maximum size
    for i in 0..MAX_SIZE {
        let key = HeaderName { inner: Repr::default() }; // Assume default is valid
        let value = TestValue;
        let hash = HashValue(i as u16);
        map.try_insert_entry(hash, key, value).unwrap();
    }

    // Now try to insert another entry which should exceed the max size
    let result = {
        let key = HeaderName { inner: Repr::default() }; // Another default key
        let value = TestValue;
        let hash = HashValue(MAX_SIZE as u16); // Hash for the new entry
        map.try_insert_entry(hash, key, value)
    };

    // Assert that the result is an error indicating maximum size reached
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), MaxSizeReached::new());
}

