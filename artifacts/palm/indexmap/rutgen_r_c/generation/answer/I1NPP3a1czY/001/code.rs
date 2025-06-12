// Answer 0

#[test]
fn test_reserve_zero_capacity() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    index_map.reserve(0);
    assert_eq!(index_map.capacity(), 0);
}

#[test]
fn test_reserve_capacity_increases() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    index_map.reserve(2);
    assert!(index_map.capacity() >= 3); // Initial 1 + additional 2
}

#[test]
fn test_reserve_capacity_not_exceeding_max() {
    let max_capacity = IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY;
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(max_capacity, RandomState::new());
    index_map.reserve(10); // Test when near max capacity
    assert!(index_map.capacity() <= max_capacity); // Ensure it does not exceed max capacity
}

#[test]
#[should_panic]
fn test_reserve_negative_capacity() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    index_map.reserve(usize::MAX); // Attempt to reserve more than max usize
}

