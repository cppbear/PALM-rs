// Answer 0

#[test]
fn test_try_reserve_exact_zero() {
    let mut index_set = IndexSet::<usize, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    index_set.try_reserve_exact(0);
}

#[test]
fn test_try_reserve_exact_minimum() {
    let mut index_set = IndexSet::<usize, RandomState>::with_capacity_and_hasher(1, RandomState::new());
    index_set.try_reserve_exact(1);
}

#[test]
fn test_try_reserve_exact_small() {
    let mut index_set = IndexSet::<usize, RandomState>::with_capacity_and_hasher(5, RandomState::new());
    index_set.try_reserve_exact(2);
}

#[test]
fn test_try_reserve_exact_large() {
    let mut index_set = IndexSet::<usize, RandomState>::with_capacity_and_hasher(100, RandomState::new());
    index_set.try_reserve_exact(1000);
}

#[test]
fn test_try_reserve_exact_near_max_capacity() {
    let mut index_set = IndexSet::<usize, RandomState>::with_capacity_and_hasher(1_000_000, RandomState::new());
    index_set.try_reserve_exact(1_000_000);
}

#[test]
fn test_try_reserve_exact_maximum() {
    let mut index_set = IndexSet::<usize, RandomState>::with_capacity_and_hasher(0, RandomState::new());
    index_set.try_reserve_exact(1_073_741_824);
}

