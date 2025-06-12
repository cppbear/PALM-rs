// Answer 0

#[test]
fn test_try_insert2_success() {
    let mut header_map: HeaderMap<String> = HeaderMap::with_capacity(1);
    let key = HeaderName { inner: Repr::Custom };
    let value = String::from("value1");
    
    let _ = header_map.try_insert2(key.clone(), value.clone());
}

#[test]
fn test_try_insert2_fail_due_to_max_size() {
    let mut header_map: HeaderMap<String> = HeaderMap::try_with_capacity(MAX_SIZE).unwrap();
    
    for i in 0..MAX_SIZE {
        let key = HeaderName { inner: Repr::Custom };
        let value = format!("value{}", i);
        let _ = header_map.try_insert2(key.clone(), value);
    }

    let key = HeaderName { inner: Repr::Custom };
    let value = String::from("additional_value");

    let result = header_map.try_insert2(key, value);
    assert!(result.is_err());
}

#[test]
fn test_try_insert2_with_non_unique_keys() {
    let mut header_map: HeaderMap<String> = HeaderMap::with_capacity(3);
    
    let key = HeaderName { inner: Repr::Custom };
    let value1 = String::from("value1");
    let value2 = String::from("value2");

    let _ = header_map.try_insert2(key.clone(), value1.clone());
    let _ = header_map.try_insert2(key.clone(), value2.clone());

    let result = header_map.try_insert2(key.clone(), value2.clone());
    assert!(result.is_ok());
}

