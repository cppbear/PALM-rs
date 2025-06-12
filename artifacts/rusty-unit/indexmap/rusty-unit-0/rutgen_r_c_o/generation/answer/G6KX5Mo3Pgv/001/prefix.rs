// Answer 0

#[test]
fn test_truncate_zero_length() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.truncate(0);
}

#[test]
fn test_truncate_below_current_length() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.truncate(2);
}

#[test]
fn test_truncate_equal_to_current_length() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.truncate(2);
}

#[test]
fn test_truncate_exceeds_current_length() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.truncate(10);
}

#[test]
fn test_truncate_max_entries_capacity_plus_one() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY + 1, RandomState::new());
    for i in 0..IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY {
        map.insert(i, i * 10);
    }
    map.truncate(IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY + 1);
}

