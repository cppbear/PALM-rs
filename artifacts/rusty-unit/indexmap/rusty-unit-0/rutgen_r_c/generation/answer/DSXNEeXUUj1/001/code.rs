// Answer 0

#[test]
fn test_index_map_core_new() {
    struct TestKey;
    struct TestValue;

    type TestIndices = hash_table::HashTable<usize>;
    type TestEntries<K, V> = Vec<Bucket<K, V>>;

    let index_map_core: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();

    assert_eq!(index_map_core.len(), 0);
    assert_eq!(index_map_core.capacity(), 0);
}

#[test]
fn test_index_map_core_new_empty() {
    struct TestKey;
    struct TestValue;

    let index_map_core: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();

    assert!(index_map_core.entries.is_empty());
}

