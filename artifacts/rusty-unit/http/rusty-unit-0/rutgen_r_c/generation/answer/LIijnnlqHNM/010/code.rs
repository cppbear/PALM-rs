// Answer 0

#[test]
fn test_try_insert_with_new_key() {
    let mut header_map: HeaderMap<i32> = HeaderMap::with_capacity(10);
    let key = HeaderName { inner: Repr::Custom("Test-Key".to_string()) };
    let value = 42;

    let result = header_map.try_insert2(key.clone(), value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(header_map.len(), 1);
}

#[test]
fn test_try_insert_with_existing_key() {
    let mut header_map: HeaderMap<i32> = HeaderMap::with_capacity(10);
    let key = HeaderName { inner: Repr::Custom("Duplicate-Key".to_string()) };
    let value1 = 42;
    let value2 = 100;

    header_map.try_insert2(key.clone(), value1).unwrap();
    let result = header_map.try_insert2(key.clone(), value2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(value1));
    assert_eq!(header_map.len(), 1);
    assert_eq!(header_map.get(key.clone()), Some(&value2));
}

#[test]
fn test_try_insert_exceeding_capacity() {
    let mut header_map: HeaderMap<i32> = HeaderMap::with_capacity(1);
    let key1 = HeaderName { inner: Repr::Custom("Key1".to_string()) };
    let key2 = HeaderName { inner: Repr::Custom("Key2".to_string()) };
    let value1 = 42;
    let value2 = 100;

    header_map.try_insert2(key1.clone(), value1).unwrap();
    let result = header_map.try_insert2(key2.clone(), value2);
    assert!(result.is_err());
}

#[test]
fn test_try_insert_with_collision() {
    let mut header_map: HeaderMap<i32> = HeaderMap::with_capacity(10);
    let key1 = HeaderName { inner: Repr::Custom("Collision-Key".to_string()) };
    let key2 = HeaderName { inner: Repr::Custom("Collision-Key".to_string()) }; // Same key, to force a collision
    let value1 = 42;

    header_map.try_insert2(key1.clone(), value1).unwrap();
    let result = header_map.try_insert2(key2.clone(), value1);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(value1));
}

#[test]
fn test_try_insert_non_empty_map() {
    let mut header_map: HeaderMap<i32> = HeaderMap::with_capacity(10);
    let key = HeaderName { inner: Repr::Custom("Non-Empty-Key".to_string()) };
    let value = 42;

    header_map.try_insert2(key.clone(), value).unwrap();
    assert!(!header_map.is_empty());

    let result = header_map.try_insert2(key.clone(), 100);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(value));
}

