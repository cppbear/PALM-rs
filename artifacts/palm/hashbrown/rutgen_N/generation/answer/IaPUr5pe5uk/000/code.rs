// Answer 0

#[test]
fn test_reserve_increases_capacity() {
    use hashbrown::HashSet;
    
    let mut set: HashSet<i32> = HashSet::new();
    set.reserve(10);
    assert!(set.capacity() >= 10);
}

#[test]
fn test_reserve_zero() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    let initial_capacity = set.capacity();
    set.reserve(0);
    assert_eq!(set.capacity(), initial_capacity);
}

#[test]
#[should_panic]
fn test_reserve_exceeds_max_capacity() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    let additional = isize::MAX as usize + 1; // This will exceed the max capacity.
    set.reserve(additional);
}

