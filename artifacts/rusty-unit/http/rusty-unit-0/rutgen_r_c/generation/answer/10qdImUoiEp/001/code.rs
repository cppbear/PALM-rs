// Answer 0

#[test]
fn test_try_insert_phase_two_success() {
    let mut header_map = HeaderMap::<HeaderValue>::with_capacity(10);
    let key = HeaderName { inner: Repr::<Custom>::new() }; // Assuming Repr has a new() method.
    let value = HeaderValue::new(); // Assuming HeaderValue has a new() method.
    let hash = HashValue(123);
    let probe = 0;
    let danger = false;

    let result = header_map.try_insert_phase_two(key.clone(), value, hash, probe, danger);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0); // Expecting index 0 after first insert
}

#[test]
fn test_try_insert_phase_two_max_size_reached() {
    let mut header_map = HeaderMap::<HeaderValue>::with_capacity(1);
    let key = HeaderName { inner: Repr::<Custom>::new() }; // Assuming Repr has a new() method.
    let value = HeaderValue::new(); // Assuming HeaderValue has a new() method.
    let hash = HashValue(123);
    let probe = 0;

    // Insert the first item successfully
    assert!(header_map.try_insert_phase_two(key.clone(), value.clone(), hash, probe, false).is_ok());

    // Now try to insert another item which should reach the max size limit
    let result = header_map.try_insert_phase_two(key.clone(), value.clone(), hash, probe, false);
    assert!(result.is_err()); // Expecting to fail due to max size reached
}

#[test]
fn test_try_insert_phase_two_danger_increased() {
    let mut header_map = HeaderMap::<HeaderValue>::with_capacity(5);
    let key = HeaderName { inner: Repr::<Custom>::new() }; // Assuming Repr has a new() method.
    let value1 = HeaderValue::new(); // Assuming HeaderValue has a new() method.
    let value2 = HeaderValue::new(); // Different value
    let hash1 = HashValue(123);
    let hash2 = HashValue(456);
    let probe = 0;

    // Insert the first item
    assert!(header_map.try_insert_phase_two(key.clone(), value1, hash1, probe, false).is_ok());

    // This insert should trigger a danger level increase due to displacement (assuming it does)
    let result = header_map.try_insert_phase_two(key, value2, hash2, probe, true);
    assert!(result.is_ok());

    // Checking if the danger level has increased to Yellow
    assert!(matches!(header_map.danger, Danger::Yellow));
}

