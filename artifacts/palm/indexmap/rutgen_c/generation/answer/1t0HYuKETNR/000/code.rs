// Answer 0

#[test]
fn test_try_reserve_exact_success() {
    use core::collections::hash_map::RandomState;

    let mut index_set: super::IndexSet<i32, RandomState> = super::IndexSet::with_capacity_and_hasher(5, RandomState::new());
    assert_eq!(index_set.len(), 0);
    
    let result = index_set.try_reserve_exact(3);
    assert!(result.is_ok());
    assert_eq!(index_set.capacity(), 5); // Ensure capacity remains the same
}

#[test]
fn test_try_reserve_exact_failure() {
    use core::collections::hash_map::RandomState;

    let mut index_set: super::IndexSet<i32, RandomState> = super::IndexSet::with_capacity_and_hasher(0, RandomState::new());
    
    let result = index_set.try_reserve_exact(usize::MAX);
    assert!(result.is_err());
}

#[test]
fn test_try_reserve_exact_no_change() {
    use core::collections::hash_map::RandomState;

    let mut index_set: super::IndexSet<i32, RandomState> = super::IndexSet::with_capacity_and_hasher(2, RandomState::new());
    index_set.try_reserve_exact(2).unwrap();
    
    // Trying to reserve an amount that doesn't require increasing the capacity
    let result = index_set.try_reserve_exact(1);
    assert!(result.is_ok());
    assert_eq!(index_set.capacity(), 2); // Ensure capacity does not change
}

