// Answer 0

#[test]
fn test_reserve_exact_zero() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    index_set.reserve_exact(0);
}

#[test]
fn test_reserve_exact_small() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    index_set.reserve_exact(3);
}

#[test]
fn test_reserve_exact_large() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    index_set.reserve_exact(100);
}

#[test]
fn test_reserve_exact_max() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    index_set.reserve_exact(usize::MAX);
}

#[test]
#[should_panic]
fn test_reserve_exact_exceed_capacity() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    index_set.reserve_exact(usize::MAX);
}

