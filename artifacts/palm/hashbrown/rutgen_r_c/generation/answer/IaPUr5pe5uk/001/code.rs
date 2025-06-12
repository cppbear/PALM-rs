// Answer 0

#[test]
fn test_reserve_valid_capacity() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    set.reserve(10);
    assert!(set.capacity() >= 10);
}

#[test]
#[should_panic]
fn test_reserve_exceeding_isize_max() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    set.reserve(std::isize::MAX as usize);
}

