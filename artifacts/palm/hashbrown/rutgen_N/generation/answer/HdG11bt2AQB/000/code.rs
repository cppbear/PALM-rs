// Answer 0

#[test]
fn test_capacity_with_initial_capacity() {
    use hashbrown::HashSet;

    let set: HashSet<i32> = HashSet::with_capacity(100);
    assert!(set.capacity() >= 100);
}

#[test]
fn test_capacity_with_zero_initial_capacity() {
    use hashbrown::HashSet;

    let set: HashSet<i32> = HashSet::with_capacity(0);
    assert_eq!(set.capacity(), 0);
}

#[test]
fn test_capacity_after_insertions() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::with_capacity(2);
    set.insert(1);
    set.insert(2);
    assert!(set.capacity() >= 2);
}

