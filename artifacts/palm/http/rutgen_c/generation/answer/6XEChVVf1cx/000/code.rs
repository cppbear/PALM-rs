// Answer 0

#[test]
fn test_try_insert_entry_success() {
    struct TestValue;

    let mut header_map = HeaderMap::<TestValue>::with_capacity(1);
    let hash_value = HashValue(1);
    let key = HeaderName { inner: Repr::Custom };

    let result = header_map.try_insert_entry(hash_value, key.clone(), TestValue);
    assert!(result.is_ok());
    assert_eq!(header_map.entries.len(), 1);
}

#[test]
fn test_try_insert_entry_max_size_reached() {
    struct TestValue;

    let mut header_map = HeaderMap::<TestValue>::with_capacity(MAX_SIZE);
    let hash_value = HashValue(1);
    let key = HeaderName { inner: Repr::Custom };

    for i in 0..MAX_SIZE {
        header_map.try_insert_entry(HashValue(i), key.clone(), TestValue).unwrap();
    }

    let result = header_map.try_insert_entry(hash_value, key.clone(), TestValue);
    assert!(result.is_err());
    assert_eq!(header_map.entries.len(), MAX_SIZE);
}

