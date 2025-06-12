// Answer 0

#[test]
fn test_try_reserve_success() {
    use hashbrown::HashSet;
    let mut set: HashSet<i32> = HashSet::new();
    assert!(set.try_reserve(10).is_ok());
}

#[test]
fn test_try_reserve_zero() {
    use hashbrown::HashSet;
    let mut set: HashSet<i32> = HashSet::new();
    assert!(set.try_reserve(0).is_ok());
}

#[should_panic]
fn test_try_reserve_capacity_overflow() {
    use hashbrown::HashSet;
    let mut set: HashSet<u32> = HashSet::new();
    let _ = set.try_reserve(usize::MAX);
}

