// Answer 0

#[test]
fn test_reverse_empty() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.reverse();
}

#[test]
fn test_reverse_single_entry() {
    let mut index_map = IndexMapCore::with_capacity(1);
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 0, value: 1 });
    index_map.indices.push(0);
    index_map.reverse();
}

#[test]
fn test_reverse_two_entries() {
    let mut index_map = IndexMapCore::with_capacity(2);
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 0, value: 1 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 2 });
    index_map.indices.push(0);
    index_map.indices.push(1);
    index_map.reverse();
}

#[test]
fn test_reverse_three_entries() {
    let mut index_map = IndexMapCore::with_capacity(3);
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 0, value: 1 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 2 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 3 });
    index_map.indices.push(0);
    index_map.indices.push(1);
    index_map.indices.push(2);
    index_map.reverse();
}

#[test]
fn test_reverse_multiple_entries() {
    let mut index_map = IndexMapCore::with_capacity(5);
    for i in 0..5 {
        index_map.entries.push(Bucket { hash: HashValue::default(), key: i, value: i });
        index_map.indices.push(i);
    }
    index_map.reverse();
}

#[test]
fn test_reverse_max_entries_capacity() {
    let mut index_map = IndexMapCore::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
    for i in 0..IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY {
        index_map.entries.push(Bucket { hash: HashValue::default(), key: i, value: i });
        index_map.indices.push(i);
    }
    index_map.reverse();
}

