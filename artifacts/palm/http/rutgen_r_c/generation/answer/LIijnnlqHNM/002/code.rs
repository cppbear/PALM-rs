// Answer 0

fn test_try_insert2_success() {
    struct DummyHeaderValue;

    let mut map: HeaderMap<DummyHeaderValue> = HeaderMap::with_capacity(16);
    let key = HeaderName { inner: Repr::Custom }; // Initialize the key
    let value = DummyHeaderValue; // Use dummy header value

    // First insert should succeed
    let result = map.try_insert2(key.clone(), value);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

fn test_try_insert2_conflict() {
    struct DummyHeaderValue;

    let mut map: HeaderMap<DummyHeaderValue> = HeaderMap::with_capacity(16);
    let key1 = HeaderName { inner: Repr::Custom }; // Initialize first key
    let key2 = HeaderName { inner: Repr::Custom }; // Initialize second key, should cause a conflict
    let value1 = DummyHeaderValue; // First header value
    let value2 = DummyHeaderValue; // Second header value

    // Insert the first key
    map.try_insert2(key1.clone(), value1).unwrap();

    // Insert the second key which should conflict
    let result = map.try_insert2(key1.clone(), value2);
    assert!(result.is_ok());
    assert!(result.unwrap().is_some()); // Should return the old value
}

fn test_try_insert2_capacity_exceed() {
    struct DummyHeaderValue;

    let mut map: HeaderMap<DummyHeaderValue> = HeaderMap::with_capacity(1);
    let key = HeaderName { inner: Repr::Custom }; // Initialize the key
    let value = DummyHeaderValue; // Use dummy header value

    // Insert the first value
    map.try_insert2(key.clone(), value).unwrap();

    // Now attempting to insert another value should hit the capacity limit
    let result = map.try_insert2(key.clone(), value);
    assert!(result.is_err()); // Should return an error indicating max size reached
}

fn test_try_insert2_should_panic() {
    struct DummyHeaderValue;

    let mut map: HeaderMap<DummyHeaderValue> = HeaderMap::with_capacity(16);
    let key = HeaderName { inner: Repr::Custom }; // Initialize the key
    let value = DummyHeaderValue; // Use dummy header value

    // Create a scenario that ensures panic
    let mut indices = vec![
        Pos::new(1, HashValue(1)), // This position is occupied
    ];
    map.indices = indices.into_boxed_slice();

    map.try_insert2(key.clone(), value).unwrap(); // Should trigger a panic due to occupied slot
}

fn test_try_insert2_forward_shift() {
    struct DummyHeaderValue;

    let mut map: HeaderMap<DummyHeaderValue> = HeaderMap::with_capacity(16);
    let key = HeaderName { inner: Repr::Custom }; // Initialize the key
    let value = DummyHeaderValue; // Use dummy header value

    // Pre-fill the map with entries to ensure forward shift condition
    for _ in 0..FORWARD_SHIFT_THRESHOLD {
        map.try_insert2(key.clone(), value).unwrap();
    }

    // Insert item that will trigger the forward shift
    let result = map.try_insert2(key.clone(), value);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none()); // Should end with a new insertion
}

