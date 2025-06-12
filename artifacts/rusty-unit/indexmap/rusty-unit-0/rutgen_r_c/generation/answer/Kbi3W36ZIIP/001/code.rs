// Answer 0

#[test]
fn test_rebuild_hash_table_empty() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.rebuild_hash_table();
    assert_eq!(index_map.indices.len(), 0);
}

#[test]
fn test_rebuild_hash_table_single_entry() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let hash = HashValue::new(42);
    index_map.entries.push(Bucket { hash, key: 1, value: 10 });
    index_map.rebuild_hash_table();
    assert_eq!(index_map.indices.len(), 1);
}

#[test]
fn test_rebuild_hash_table_multiple_entries() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let hash1 = HashValue::new(42);
    let hash2 = HashValue::new(84);
    index_map.entries.push(Bucket { hash: hash1, key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: hash2, key: 2, value: 20 });
    index_map.rebuild_hash_table();
    assert_eq!(index_map.indices.len(), 2);
}

#[should_panic(expected = "assertion failed")]
#[test]
fn test_rebuild_hash_table_panic_on_invariant() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let hash = HashValue::new(42);
    index_map.entries.push(Bucket { hash, key: 1, value: 10 });
    // Manually manipulating the indices to set up for a panic.
    index_map.indices.insert_unique(hash.get(), 0, |_| unreachable!());
    index_map.rebuild_hash_table();
} 

#[test]
fn test_rebuild_hash_table_after_clear() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let hash = HashValue::new(42);
    index_map.entries.push(Bucket { hash, key: 1, value: 10 });
    index_map.rebuild_hash_table();
    index_map.clear();
    index_map.rebuild_hash_table();
    assert_eq!(index_map.indices.len(), 0);
}

