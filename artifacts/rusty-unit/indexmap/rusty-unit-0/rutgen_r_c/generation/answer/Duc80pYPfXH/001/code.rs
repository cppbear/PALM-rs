// Answer 0

#[test]
fn test_shrink_to_smaller_capacity() {
    use std::collections::hash_map::RandomState;

    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    set.reserve(10);
    assert_eq!(set.capacity(), 10);
    
    set.shrink_to(5);
    assert!(set.capacity() >= 5);
}

#[test]
fn test_shrink_to_exact_capacity() {
    use std::collections::hash_map::RandomState;

    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    set.reserve(10);
    assert_eq!(set.capacity(), 10);
    
    set.shrink_to(10);
    assert_eq!(set.capacity(), 10);
}

#[test]
fn test_shrink_to_zero_capacity() {
    use std::collections::hash_map::RandomState;

    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    set.reserve(10);
    assert_eq!(set.capacity(), 10);
    
    set.shrink_to(0);
    assert!(set.capacity() >= 0);
}

#[test]
fn test_shrink_to_capacity_over_min() {
    use std::collections::hash_map::RandomState;

    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    set.reserve(10);
    assert_eq!(set.capacity(), 10);
    
    set.shrink_to(3);
    assert!(set.capacity() >= 3);
} 

#[should_panic]
fn test_shrink_to_panic() {
    use std::collections::hash_map::RandomState;

    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    set.shrink_to(1); // This should panic as it cannot shrink below current capacity
} 

#[test]
fn test_shrink_to_no_effect() {
    use std::collections::hash_map::RandomState;

    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    assert_eq!(set.capacity(), 5);
    
    set.shrink_to(5); // Attempt to shrink to current capacity
    assert_eq!(set.capacity(), 5); // Ensure capacity remains unchanged
}

