// Answer 0

#[test]
fn test_insert_into_empty_map() {
    let mut map = HeaderMap::with_capacity(10);
    let key = HeaderName::from_static("test-key");
    let value = HeaderValue::from_static("test-value");

    assert!(map.insert(key.clone(), value.clone()).is_none());
    assert_eq!(map.len(), 1);
}

#[test]
fn test_insert_overwriting_existing_key() {
    let mut map = HeaderMap::with_capacity(10);
    let key = HeaderName::from_static("test-key");
    let value1 = HeaderValue::from_static("value1");
    let value2 = HeaderValue::from_static("value2");

    assert!(map.insert(key.clone(), value1.clone()).is_none());
    assert_eq!(map.len(), 1);

    let previous_value = map.insert(key.clone(), value2.clone()).unwrap();
    assert_eq!(previous_value, value1);
    assert_eq!(map.len(), 1);
}

#[test]
#[should_panic]
fn test_insert_panics_on_exceeding_max_capacity() {
    let mut map = HeaderMap::try_with_capacity(MAX_SIZE as usize).unwrap();

    for i in 0..MAX_SIZE {
        let key = HeaderName::from_static(&format!("key-{}", i));
        let value = HeaderValue::from_static(&format!("value-{}", i));
        map.insert(key, value);
    }

    // This insert should panic as it exceeds max capacity.
    let extra_key = HeaderName::from_static("extra-key");
    let extra_value = HeaderValue::from_static("extra-value");
    map.insert(extra_key, extra_value);
}

