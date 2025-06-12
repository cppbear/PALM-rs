// Answer 0

#[test]
fn test_try_append_empty_map() {
    let mut map: HeaderMap<i32> = HeaderMap::with_capacity(8);
    let key = HeaderName { inner: Repr::Custom };
    assert_eq!(map.try_append(key.clone(), 42).unwrap(), false);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_try_append_occupied_key() {
    let mut map: HeaderMap<i32> = HeaderMap::with_capacity(8);
    let key = HeaderName { inner: Repr::Custom };
    map.try_append(key.clone(), 42).unwrap();
    assert_eq!(map.try_append(key.clone(), 84).unwrap(), true);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_try_append_exceeding_max_size() {
    let mut map: HeaderMap<i32> = HeaderMap::with_capacity(1);
    let key1 = HeaderName { inner: Repr::Custom };
    let key2 = HeaderName { inner: Repr::Custom };
    let _ = map.try_append(key1.clone(), 42);
    assert!(map.try_append(key2.clone(), 84).is_err());
    assert!(matches!(map.try_append(key2.clone(), 84), Err(MaxSizeReached)));
}

#[test]
fn test_try_append_robinhood_scenario() {
    let mut map: HeaderMap<i32> = HeaderMap::with_capacity(8);
    let key1 = HeaderName { inner: Repr::Custom };
    let key2 = HeaderName { inner: Repr::Custom };
    
    map.try_append(key1.clone(), 42).unwrap();
    map.try_append(key2.clone(), 84).unwrap(); // Let's assume it causes a robinhood
    
    // Check that both keys are present
    assert!(map.contains_key(key1.clone()));
    assert!(map.contains_key(key2.clone()));
    assert_eq!(map.len(), 2);
}

