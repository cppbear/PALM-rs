// Answer 0

#[test]
fn test_try_insert_entry_exceed_max_size() {
    let mut header_map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(32768);
    // Filling the HeaderMap to its maximum size
    for i in 0..32768 {
        let hash = HashValue(i as u16);
        let key = HeaderName { inner: Repr::<Custom>::default() }; // Assuming Repr<Custom> has a default implementation
        let value = HeaderValue::default(); // Assuming HeaderValue has a default implementation
        header_map.try_insert_entry(hash, key, value).unwrap();
    }
    // Attempting to insert another entry should exceed max size
    let hash = HashValue(32768 as u16);
    let key = HeaderName { inner: Repr::<Custom>::default() }; 
    let value = HeaderValue::default(); 
    let result = header_map.try_insert_entry(hash, key, value);
    // The result should be an error indicating MaxSizeReached
    match result {
        Err(err) => { /* expected behavior */ }
        _ => panic!("Expected Err(MaxSizeReached) but got {:?}", result),
    }
}

