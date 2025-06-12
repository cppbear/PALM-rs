// Answer 0

#[test]
fn test_reserve_zero_capacity() {
    let hash_builder = RandomState::new();
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, hash_builder);
    index_map.reserve(0);
}

#[test]
fn test_reserve_small_capacity() {
    let hash_builder = RandomState::new();
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, hash_builder);
    index_map.reserve(2);
}

#[test]
fn test_reserve_large_capacity() {
    let hash_builder = RandomState::new();
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, hash_builder);
    index_map.reserve(usize::MAX);
}

#[test]
fn test_reserve_max_capacity() {
    let hash_builder = RandomState::new();
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(usize::MAX / 2, hash_builder);
    index_map.reserve(usize::MAX / 2);
}

