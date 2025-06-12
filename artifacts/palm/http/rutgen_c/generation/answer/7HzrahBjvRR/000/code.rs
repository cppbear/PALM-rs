// Answer 0

#[test]
fn test_remove_entry_mult() {
    struct TestValue(u32);
    let mut map = HeaderMap::<TestValue>::with_capacity(4);
    
    let key = HeaderName { inner: Repr::Custom }; // Assuming default initialization.
    map.insert(key.clone(), TestValue(1));
    map.insert(key.clone(), TestValue(2));
    
    let (removed_key, drain) = {
        let mut entry = map.entry(key.clone()).or_insert(TestValue(3));
        entry.insert(TestValue(3)); // Insert another value to be removed.
        entry.remove_entry_mult()
    };
    
    assert_eq!(removed_key, key);
    
    let values: Vec<_> = drain.into_iter().collect();
    assert_eq!(values.len(), 3);
    assert!(map.is_empty());
}

#[test]
fn test_remove_entry_mult_empty_map() {
    struct TestValue(u32);
    let mut map = HeaderMap::<TestValue>::with_capacity(4);
    
    let key = HeaderName { inner: Repr::Custom }; // Assuming default initialization.
    
    let result = std::panic::catch_unwind(|| {
        let drain = {
            let entry = map.entry(key.clone());
            entry.remove_entry_mult()
        };
    });
    
    assert!(result.is_err());
}

#[test]
fn test_remove_entry_mult_non_existing_key() {
    struct TestValue(u32);
    let mut map = HeaderMap::<TestValue>::with_capacity(4);
    
    let key = HeaderName { inner: Repr::Custom }; // Assuming default initialization.

    let drain = {
        // Attempt to remove from an empty map
        let entry = map.entry(key.clone());
        entry.remove_entry_mult()
    };
    
    assert!(drain.first.is_none());
}

