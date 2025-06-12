// Answer 0

#[test]
fn test_capacity_empty_map() {
    let map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _ = map.capacity();
}

#[test]
fn test_capacity_single_element_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.insert(1, 10);
    let _ = map.capacity();
}

#[test]
fn test_capacity_max_entries_capacity() {
    let max_capacity = IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY;
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(max_capacity, RandomState::new());
    for i in 0..max_capacity {
        map.insert(i, i);
    }
    let _ = map.capacity();
}

#[test]
fn test_capacity_exceeding_max_capacity() {
    let max_capacity = IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY + 1;
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(max_capacity, RandomState::new());
    let _ = map.capacity();
}

#[test]
fn test_capacity_after_shrink_to_fit() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    for i in 0..5 {
        map.insert(i, i * 10);
    }
    map.truncate(3);
    map.shrink_to_fit();
    let _ = map.capacity();
}

