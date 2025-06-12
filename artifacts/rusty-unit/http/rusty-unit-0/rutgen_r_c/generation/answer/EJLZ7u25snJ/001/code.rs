// Answer 0

#[test]
fn test_try_insert_new_key() {
    let mut map = HeaderMap::with_capacity(10);
    let result = map.try_insert("key1", "value1".to_string());
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
    assert_eq!(map.len(), 1);
}

#[test]
fn test_try_insert_existing_key() {
    let mut map = HeaderMap::with_capacity(10);
    map.try_insert("key1", "value1".to_string()).unwrap();
    let result = map.try_insert("key1", "value2".to_string());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("value1".to_string()));
    assert_eq!(map.len(), 1);
}

#[test]
fn test_try_insert_over_capacity() {
    let mut map = HeaderMap::try_with_capacity(1).unwrap();
    map.try_insert("key1", "value1".to_string()).unwrap();
    let result = map.try_insert("key2", "value2".to_string());
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_try_insert_panic_on_max_capacity() {
    let mut map = HeaderMap::try_with_capacity(1).unwrap();
    map.try_insert("key1", "value1".to_string()).unwrap();
    let _ = map.try_insert("key2", "value2".to_string()).unwrap(); // Should panic here
}

#[test]
fn test_try_insert_multiple_same_keys() {
    let mut map = HeaderMap::with_capacity(3);
    let result1 = map.try_insert("key1", "value1".to_string());
    let result2 = map.try_insert("key1", "value2".to_string());

    assert!(result1.is_ok());
    assert!(result1.unwrap().is_none());
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), Some("value1".to_string()));
    assert_eq!(map.len(), 1);
}

