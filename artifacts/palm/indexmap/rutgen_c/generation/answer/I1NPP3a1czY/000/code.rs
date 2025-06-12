// Answer 0

#[test]
fn test_reserve_capacity_zero() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    index_map.reserve(0);
    assert_eq!(index_map.capacity(), 0);
}

#[test]
fn test_reserve_capacity_increases() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    index_map.reserve(3);
    assert!(index_map.capacity() >= 3);
}

#[test]
fn test_reserve_capacity_exceeding() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    index_map.reserve(10);
    assert!(index_map.capacity() >= 10);
} 

#[test]
fn test_reserve_capacity_negative_effects() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    let initial_capacity = index_map.capacity();
    index_map.reserve(0); // This should have no effect
    assert_eq!(index_map.capacity(), initial_capacity);
}

#[test]
fn test_reserve_capacity_edge_case_large() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let large_number = 1 << 30; // A large number
    index_map.reserve(large_number);
    assert!(index_map.capacity() >= large_number);
}

