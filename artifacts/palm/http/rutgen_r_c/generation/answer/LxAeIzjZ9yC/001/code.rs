// Answer 0

#[test]
fn test_insert_entry_success() {
    struct TestHeaderValue;
    
    let mut map = HeaderMap::new();
    let key = HeaderName { inner: Repr::Custom };
    let value = TestHeaderValue;

    if let VacantEntry { map: ref mut map_ref, key: ref key_ref, .. } = map.try_entry(key.clone()).unwrap() {
        let occupied_entry = map_ref.insert_entry(value).expect("Failed to insert entry");
        assert_eq!(occupied_entry.index, 0);
    }
}

#[test]
#[should_panic(expected = "size overflows MAX_SIZE")]
fn test_insert_entry_panic_max_size() {
    struct TestHeaderValue;

    let mut map = HeaderMap::new();
    let key = HeaderName { inner: Repr::Custom };

    // Fill the map to max size
    for _ in 0..(MAX_SIZE as usize) {
        let value = TestHeaderValue;
        if let VacantEntry { map: ref mut map_ref, key: ref key_ref, .. } = map.try_entry(key.clone()).unwrap() {
            map_ref.try_insert_entry(value).unwrap();
        }
    }

    // This should panic because it exceeds the max size
    let value = TestHeaderValue;
    let _ = map.try_entry(key.clone()).unwrap().try_insert_entry(value);
}

