// Answer 0

#[test]
fn test_reserve_exact_zero_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    map.reserve_exact(0);
    assert_eq!(map.capacity(), 0); // Ensure capacity remains zero
}

#[test]
fn test_reserve_exact_non_zero_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(2, RandomState::new());
    map.reserve_exact(2);
    assert!(map.capacity() >= 2); // Ensure capacity is at least 2
}

#[test]
fn test_reserve_exact_large_request() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.reserve_exact(100);
    assert!(map.capacity() >= 100); // Ensure the capacity is equal or exceeds the requested amount
}

#[should_panic(expected = "some panic message related to allocation")]
fn test_reserve_exact_exceed_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY, RandomState::new());
    map.reserve_exact(1); // Should panic as we are at max capacity
} 

#[test]
fn test_reserve_exact_no_overallotment() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    let initial_capacity = map.capacity();
    map.reserve_exact(2);
    assert_eq!(map.capacity(), initial_capacity); // Ensure no over-allotment occurs
}

