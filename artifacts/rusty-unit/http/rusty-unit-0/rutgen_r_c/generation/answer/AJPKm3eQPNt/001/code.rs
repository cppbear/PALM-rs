// Answer 0

#[test]
fn test_entry_insert_basic() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(10);
    let key1 = "header-1".to_string();
    let key2 = "header-2".to_string();

    let entry1 = map.entry(key1.clone()).or_insert(0);
    *entry1 += 1;

    let entry2 = map.entry(key2.clone()).or_insert(0);
    *entry2 += 5;

    assert_eq!(map.get(key1).unwrap(), &1);
    assert_eq!(map.get(key2).unwrap(), &5);
}

#[test]
fn test_entry_panic_exceeds_max_size() {
    // Assuming MAX_SIZE is determined to be 32768 for u32 entries
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(32768);
    
    for i in 0..32768 {
        let _ = map.entry(i.to_string()).or_insert(0);
    }

    let result = std::panic::catch_unwind(|| {
        let _ = map.entry("overflow-header".to_string()).or_insert(0);
    });

    assert!(result.is_err());
}

#[test]
fn test_entry_with_existing_key() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(5);
    let header1 = "existing-header".to_string();

    let entry = map.entry(header1.clone()).or_insert(0);
    *entry += 2;

    let same_entry = map.entry(header1.clone()).or_insert(0);
    *same_entry += 3;

    assert_eq!(map.get(header1).unwrap(), &5);
}

#[test]
fn test_entry_multiple_types() {
    let mut map: HeaderMap<i32> = HeaderMap::with_capacity(3);
    let key1 = "key-int".to_string();
    let key2 = "key-another-int".to_string();

    let entry1 = map.entry(key1.clone()).or_insert(10);
    *entry1 += 5;

    let entry2 = map.entry(key2.clone()).or_insert(20);
    *entry2 += 15;

    assert_eq!(map.get(key1).unwrap(), &15);
    assert_eq!(map.get(key2).unwrap(), &35);
}

