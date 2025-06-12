// Answer 0

#[test]
fn test_rebuild_hash_table_empty() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.rebuild_hash_table();
}

#[test]
fn test_rebuild_hash_table_single_entry() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 2 });
    index_map.rebuild_hash_table();
}

#[test]
fn test_rebuild_hash_table_multiple_entries() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 2 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 3, value: 4 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 5, value: 6 });
    index_map.rebuild_hash_table();
}

#[test]
fn test_rebuild_hash_table_max_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    for i in 0..(IndexMapCore::MAX_ENTRIES_CAPACITY) {
        index_map.entries.push(Bucket { hash: HashValue::default(), key: i, value: i + 1 });
    }
    index_map.rebuild_hash_table();
}

#[should_panic]
fn test_rebuild_hash_table_exceeding_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 2 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 3 });
    index_map.rebuild_hash_table();
}

