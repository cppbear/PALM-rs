// Answer 0

#[test]
fn test_try_insert_entry_valid_case() {
    let mut header_map: HeaderMap<u32> = HeaderMap::with_capacity(10);
    let hash = HashValue(1);
    let key = HeaderName { inner: Repr::custom() };
    let value = 42;
    header_map.try_insert_entry(hash, key, value).unwrap();
}

#[test]
fn test_try_insert_entry_at_max_capacity() {
    let mut header_map: HeaderMap<u32> = HeaderMap::with_capacity(32767);
    let hash = HashValue(1);
    let key = HeaderName { inner: Repr::custom() };
    let value = 42;
    for i in 0..32767 {
        header_map.try_insert_entry(HashValue(i as u16), key.clone(), value).unwrap();
    }
    
    let err = header_map.try_insert_entry(hash, key, value);
    assert!(err.is_err());
}

#[test]
fn test_try_insert_entry_with_varied_hashes() {
    let mut header_map: HeaderMap<u32> = HeaderMap::with_capacity(15);
    for i in 0..15 {
        let hash = HashValue(i as u16);
        let key = HeaderName { inner: Repr::custom() };
        let value = i * 10;
        header_map.try_insert_entry(hash, key, value).unwrap();
    }
}

#[test]
fn test_try_insert_entry_non_panic_with_different_values() {
    let mut header_map: HeaderMap<String> = HeaderMap::with_capacity(5);
    let hash = HashValue(100);
    let key = HeaderName { inner: Repr::custom() };
    let values = vec!["value1".to_string(), "value2".to_string(), "value3".to_string()];

    for value in values {
        header_map.try_insert_entry(hash, key.clone(), value).unwrap();
    }
}

#[test]
fn test_try_insert_entry_edge_case_reaching_limit() {
    let mut header_map: HeaderMap<u32> = HeaderMap::with_capacity(32768);
    let hash = HashValue(1);
    let key = HeaderName { inner: Repr::custom() };

    for i in 0..32768 {
        if i < 32767 {
            header_map.try_insert_entry(HashValue(i as u16), key.clone(), i).unwrap();
        } else {
            let err = header_map.try_insert_entry(hash, key.clone(), i);
            assert!(err.is_err());
        }
    }
}

