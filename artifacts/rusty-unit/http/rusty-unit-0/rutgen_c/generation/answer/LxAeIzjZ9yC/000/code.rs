// Answer 0

#[test]
fn test_insert_entry_creates_occupied_entry() {
    struct TestHeaderValue(String);
    
    let mut map = HeaderMap::new();
    let key = HeaderName { inner: Repr::<Custom>::default() };
    
    if let VacantEntry { map: _, key: entry_key, hash: _, probe: _, danger: _ } = 
        map.try_entry(key).unwrap() {
        let occupied_entry = entry_key.insert(TestHeaderValue("value1".into()));
        
        assert_eq!(occupied_entry.map.entries.len(), 1);
        assert_eq!(occupied_entry.map.entries[0].value, TestHeaderValue("value1".into()));
    } else {
        panic!("Expected VacantEntry, found something else");
    }
}

#[test]
fn test_insert_entry_overflow() {
    struct TestHeaderValue(String);
    
    let mut map = HeaderMap::new();
    let key = HeaderName { inner: Repr::<Custom>::default() };
    
    for _ in 0..MAX_SIZE as usize {
        let _ = map.try_entry(key).unwrap().insert(TestHeaderValue("value".into()));
    }
    
    let result = map.try_entry(key).unwrap().insert(TestHeaderValue("overflow".into()));
    
    assert_eq!(result.is_err(), true);
}

