// Answer 0

#[test]
fn test_with_capacity_in_non_zero_capacity() {
    use hashbrown::HashSet;
    use std::alloc::Global;

    let set: HashSet<i32> = HashSet::with_capacity_in(10, Global);
    assert!(set.capacity() >= 10);
}

#[test]
fn test_with_capacity_in_zero_capacity() {
    use hashbrown::HashSet;
    use std::alloc::Global;

    let set: HashSet<i32> = HashSet::with_capacity_in(0, Global);
    assert!(set.capacity() == 0);
}

#[test]
fn test_with_capacity_in_small_capacity() {
    use hashbrown::HashSet;
    use std::alloc::Global;

    let set: HashSet<String> = HashSet::with_capacity_in(1, Global);
    assert!(set.capacity() >= 1);
}

