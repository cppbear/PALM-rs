// Answer 0

#[test]
fn test_shrink_to_fit_empty() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    set.shrink_to_fit();
    assert_eq!(set.capacity(), 0);
}

#[test]
fn test_shrink_to_fit_non_empty() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    set.reserve(5); // Simulate adding 5 elements, which changes capacity
    set.shrink_to_fit();
    assert!(set.capacity() <= 5);
}

#[test]
fn test_shrink_to_fit_no_op() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    set.shrink_to_fit();
    assert_eq!(set.capacity(), 5);
}

