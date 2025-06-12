// Answer 0

#[test]
fn test_try_append2_success() {
    struct SimpleValue(u32);
    
    let mut header_map: HeaderMap<SimpleValue> = HeaderMap::with_capacity(4);
    let key = HeaderName { inner: Repr::default() };
    let value = SimpleValue(42);
    
    let result = header_map.try_append2(key.clone(), value);
    
    assert_eq!(result.unwrap(), false); // Expecting to insert as vacant
    assert_eq!(header_map.len(), 1); // Should contain one entry
}

#[test]
fn test_try_append2_over_capacity() {
    struct SimpleValue(u32);
    
    let mut header_map: HeaderMap<SimpleValue> = HeaderMap::with_capacity(1);
    let key1 = HeaderName { inner: Repr::default() };
    let value1 = SimpleValue(42);
    
    header_map.try_append2(key1.clone(), value1).unwrap();
    
    let key2 = HeaderName { inner: Repr::default() }; // Duplicate to trigger panic
    let value2 = SimpleValue(43);
    
    // Force the header map to hit capacity and expect a failure
    let result = header_map.try_append2(key2, value2);
    
    assert!(result.is_err()); // Expecting Out of Capacity panic
}

#[test]
fn test_try_append2_entry_exists() {
    struct SimpleValue(u32);
    
    let mut header_map: HeaderMap<SimpleValue> = HeaderMap::with_capacity(4);
    let key = HeaderName { inner: Repr::default() };
    let value1 = SimpleValue(42);
    
    header_map.try_append2(key.clone(), value1).unwrap();
    
    let value2 = SimpleValue(43);
    
    let result = header_map.try_append2(key.clone(), value2);
    
    assert_eq!(result.unwrap(), true); // Expecting to append to occupied
    assert_eq!(header_map.len(), 1); // Still should contain one entry
}

#[test]
#[should_panic]
fn test_try_append2_invalid_key() {
    struct SimpleValue(u32);
    
    // Using a HeaderMap with very small capacity
    let mut header_map: HeaderMap<SimpleValue> = HeaderMap::with_capacity(1);
    
    let key = HeaderName { inner: Repr::default() };
    let value = SimpleValue(42);
    
    header_map.try_append2(key.clone(), value).unwrap();
    
    // This action should trigger a panic due to maximum size reached
    header_map.try_append2(key, value).unwrap(); 
}

