// Answer 0

#[test]
fn test_try_reserve_exact_zero_capacity() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _ = index_map.try_reserve_exact(0);
}

#[test]
fn test_try_reserve_exact_one_capacity() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _ = index_map.try_reserve_exact(1);
}

#[test]
fn test_try_reserve_exact_max_capacity() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _ = index_map.try_reserve_exact(usize::MAX);
}

#[test]
fn test_try_reserve_exact_max_minus_one_capacity() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _ = index_map.try_reserve_exact(usize::MAX - 1);
}

#[test]
fn test_try_reserve_exact_half_max_capacity() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _ = index_map.try_reserve_exact(usize::MAX / 2);
}

#[test]
fn test_try_reserve_exact_zero_capacity_after_max() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _ = index_map.try_reserve_exact(0);
    let _ = index_map.try_reserve_exact(usize::MAX);
}

#[test]
fn test_try_reserve_exact_zero_capacity_again() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _ = index_map.try_reserve_exact(1);
    let _ = index_map.try_reserve_exact(0);
}

#[test]
#[should_panic]
fn test_try_reserve_exact_invalid_negative() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _ = index_map.try_reserve_exact(usize::MAX);
    let _ = index_map.try_reserve_exact(usize::MAX);
}

