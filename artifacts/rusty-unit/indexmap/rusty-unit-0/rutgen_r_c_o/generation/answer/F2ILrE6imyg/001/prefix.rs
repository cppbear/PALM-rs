// Answer 0

#[test]
fn test_values_empty() {
    let index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let values_iter = index_map.values();
}

#[test]
fn test_values_single_entry() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    index_map.insert(1, 10);
    let values_iter = index_map.values();
}

#[test]
fn test_values_multiple_entries() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    index_map.insert(1, 10);
    index_map.insert(2, 20);
    index_map.insert(3, 30);
    let values_iter = index_map.values();
}

#[test]
fn test_values_large_capacity() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1_000_000, RandomState::new());
    for i in 0..1_000_000 {
        index_map.insert(i, i * 10);
    }
    let values_iter = index_map.values();
}

#[test]
fn test_values_after_clear() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    index_map.insert(1, 10);
    index_map.insert(2, 20);
    index_map.clear();
    let values_iter = index_map.values();
}

#[test]
fn test_values_after_truncate() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    index_map.insert(1, 10);
    index_map.insert(2, 20);
    index_map.truncate(1);
    let values_iter = index_map.values();
}

