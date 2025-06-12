// Answer 0

#[test]
fn test_reserve_with_capacity_increase() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(2);
    index_map.entries.push(Bucket { hash: HashValue::new(1), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: HashValue::new(2), key: 2, value: 20 });
    index_map.reserve(2);
}

#[test]
fn test_reserve_edge_case() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.reserve(1);
}

#[test]
fn test_reserve_large_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    for i in 0..10 {
        index_map.entries.push(Bucket { hash: HashValue::new(i), key: i, value: i * 10 });
    }
    index_map.reserve(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY - 9);
} 

#[test]
fn test_reserve_exceeding_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    index_map.entries.push(Bucket { hash: HashValue::new(1), key: 1, value: 100 });
    index_map.entries.push(Bucket { hash: HashValue::new(2), key: 2, value: 200 });
    index_map.reserve(6);
}

#[test]
#[should_panic]
fn test_reserve_panics_on_excessive_request() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.reserve(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY + 1);
}

