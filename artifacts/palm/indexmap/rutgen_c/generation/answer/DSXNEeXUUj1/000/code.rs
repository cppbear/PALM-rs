// Answer 0

#[test]
fn test_index_map_core_new() {
    struct TestKey;
    struct TestValue;

    type TestIndices = hash_table::HashTable<usize>;
    type TestEntries<K, V> = Vec<Bucket<K, V>>;

    let map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();

    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_index_map_core_with_capacity() {
    struct TestKey;
    struct TestValue;

    let capacity = 10;
    let map: IndexMapCore<TestKey, TestValue> = IndexMapCore::with_capacity(capacity);

    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), capacity);
}

