// Answer 0

#[test]
fn test_try_insert_phase_two_success() {
    use crate::header::map::{HeaderMap, HashValue, HeaderName, MaxSizeReached};

    // Define a struct to use as the value type for the HeaderMap
    struct TestValue;

    // Create a HeaderMap with enough capacity
    let mut header_map: HeaderMap<TestValue> = HeaderMap::with_capacity(512); // capacity > DISPLACEMENT_THRESHOLD

    // Define a valid HashValue and HeaderName
    let hash_value = HashValue(1);
    let header_name = HeaderName { inner: Default::default() };

    // Insert an initial value to occupy the first position
    header_map.try_insert_entry(hash_value, header_name.clone(), TestValue).unwrap();

    // Invoke the function with the required conditions
    let result = header_map.try_insert_phase_two(header_name, TestValue, hash_value, 0, false);

    // Assert that the result is Ok with the expected index
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1); // The index should be 1 since we inserted one element before
}

#[test]
#[should_panic(expected = "MaxSizeReached")]
fn test_try_insert_phase_two_panic_on_full_capacity() {
    use crate::header::map::{HeaderMap, HashValue, HeaderName, MaxSizeReached};

    // Define a struct to use as the value type for the HeaderMap
    struct TestValue;

    // Create a HeaderMap with a small capacity to trigger MaxSizeReached
    let mut header_map: HeaderMap<TestValue> = HeaderMap::with_capacity(1);

    // Define a valid HashValue and HeaderName
    let hash_value = HashValue(1);
    let header_name = HeaderName { inner: Default::default() };

    // Fill the map to its maximum size
    header_map.try_insert_entry(hash_value, header_name.clone(), TestValue).unwrap();

    // Attempt to insert another entry, which should panic
    let _ = header_map.try_insert_phase_two(header_name, TestValue, hash_value, 0, false);
}

#[test]
fn test_try_insert_phase_two_displacement_threshold_not_met() {
    use crate::header::map::{HeaderMap, HashValue, HeaderName};

    // Define a struct to use as the value type for the HeaderMap
    struct TestValue;

    // Create a HeaderMap
    let mut header_map: HeaderMap<TestValue> = HeaderMap::with_capacity(512); // capacity > DISPLACEMENT_THRESHOLD

    // Define a valid HashValue and HeaderName
    let hash_value = HashValue(1);
    let header_name = HeaderName { inner: Default::default() };

    // Fill the map with enough values to ensure num_displaced < DISPLACEMENT_THRESHOLD
    for i in 0..128 {
        header_map.try_insert_entry(HashValue(i as u16), HeaderName { inner: Default::default() }, TestValue).unwrap();
    }

    // Try another insert that won't trigger the DISPLACEMENT_THRESHOLD condition
    let result = header_map.try_insert_phase_two(header_name, TestValue, hash_value, false);

    // Assert that the result is Ok with the expected index
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 128); // The index should be 128 for the new insertion
}

