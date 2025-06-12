// Answer 0

#[test]
fn test_try_reserve_exact_with_zero_additional_capacity() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let result = map.try_reserve_exact(0);
    assert!(result.is_ok());
}

#[test]
fn test_try_reserve_exact_with_non_zero_capacity() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(2, RandomState::new());
    let result = map.try_reserve_exact(3);
    assert!(result.is_ok());
    assert_eq!(map.core.capacity(), 5);  // Assuming the implementation follows typical capacity growth heuristics
}

#[test]
#[should_panic(expected = "attempt to reserve more than max capacity")]
fn test_try_reserve_exact_exceeding_max_capacity() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(IndexMapCore::<u32, u32>::MAX_ENTRIES_CAPACITY, RandomState::new());
    let _ = map.try_reserve_exact(1); // Will trigger panic
}

#[test]
fn test_try_reserve_exact_on_empty_map() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let result = map.try_reserve_exact(10);
    assert!(result.is_ok());
    assert!(map.core.capacity() >= 10);
}

#[test]
fn test_try_reserve_exact_with_exact_fit() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    let _ = map.try_reserve_exact(5);
    assert_eq!(map.core.capacity(), 10);  // Assuming the growth strategy
}

