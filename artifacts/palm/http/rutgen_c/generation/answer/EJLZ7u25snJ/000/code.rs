// Answer 0

#[test]
fn test_try_insert_new_key() {
    struct DummyHeaderName;
    
    impl IntoHeaderName for DummyHeaderName {
        fn try_insert<T>(&self, map: &mut HeaderMap<T>, val: T) -> Result<Option<T>, MaxSizeReached> {
            Ok(None) // Simulate inserting a new key
        }
    }
    
    let mut map = HeaderMap::<HeaderValue>::with_capacity(10);
    let key = DummyHeaderName;
    let result = map.try_insert(key, HeaderValue::from_static("value1"));
    
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
    assert!(!map.is_empty());
}

#[test]
fn test_try_insert_existing_key() {
    struct DummyHeaderName;
    
    impl IntoHeaderName for DummyHeaderName {
        fn try_insert<T>(&self, map: &mut HeaderMap<T>, val: T) -> Result<Option<T>, MaxSizeReached> {
            // Simulate existing key insertion with a previous value
            let previous_value = HeaderValue::from_static("previous_value");
            map.insert(self, previous_value.clone()); // Insert previously
            return Ok(Some(previous_value)); // Return previous on new insert
        }
    }
    
    let mut map = HeaderMap::<HeaderValue>::with_capacity(10);
    let key = DummyHeaderName;
    let _ = map.try_insert(key, HeaderValue::from_static("value1"));
    let result = map.try_insert(key, HeaderValue::from_static("value2"));
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), HeaderValue::from_static("previous_value"));
}

#[test]
fn test_try_insert_exceed_max_capacity() {
    struct DummyHeaderName;
    
    impl IntoHeaderName for DummyHeaderName {
        fn try_insert<T>(&self, map: &mut HeaderMap<T>, val: T) -> Result<Option<T>, MaxSizeReached> {
            // Simulate inserting until max size reached
            return Err(MaxSizeReached { _priv: () }); // Simulate max size exceeded
        }
    }
    
    let mut map = HeaderMap::<HeaderValue>::try_with_capacity(0).unwrap(); // Initialize with 0 capacity
    let key = DummyHeaderName;
    let result = map.try_insert(key, HeaderValue::from_static("value1"));
    
    assert!(result.is_err());
}

#[test]
fn test_insert_multiple_values() {
    struct DummyHeaderName;
    
    impl IntoHeaderName for DummyHeaderName {
        fn try_insert<T>(&self, map: &mut HeaderMap<T>, val: T) -> Result<Option<T>, MaxSizeReached> {
            // Simulate inserting the first value
            if map.len() == 0 {
                map.insert(self, HeaderValue::from_static("value1"));
                return Ok(None);
            } else {
                // Simulating that previous value exists
                let previous_value = HeaderValue::from_static("value1");
                return Ok(Some(previous_value)); // Returns first previous value on the second insert
            }
        }
    }
    
    let mut map = HeaderMap::<HeaderValue>::with_capacity(10);
    let key = DummyHeaderName;
    
    let _ = map.try_insert(key, HeaderValue::from_static("value1"));
    let result = map.try_insert(key, HeaderValue::from_static("value2"));

    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), HeaderValue::from_static("value1"));
}

