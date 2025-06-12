// Answer 0

#[test]
fn test_rebuild_hash_table_empty() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let mut map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    map.rebuild_hash_table();
    
    assert_eq!(map.indices.len(), 0);
    assert_eq!(map.entries.len(), 0);
}

#[test]
fn test_rebuild_hash_table_with_entries() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let mut map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(1) });
    map.entries.push(Bucket { hash: HashValue::default(), key: TestKey(2), value: TestValue(2) });

    map.rebuild_hash_table();

    assert_eq!(map.indices.len(), 2);
    assert_eq!(map.entries.len(), 2);
}

#[test]
fn test_rebuild_hash_table_after_clear() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let mut map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(1) });
    map.rebuild_hash_table();

    map.entries.clear();
    map.rebuild_hash_table();

    assert_eq!(map.indices.len(), 0);
    assert_eq!(map.entries.len(), 0);
}

