// Answer 0

#[test]
fn test_shrink_to_valid_capacity() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    assert!(set.capacity() >= 100);
    set.shrink_to(10);
    assert!(set.capacity() >= 10);
}

#[test]
fn test_shrink_to_zero_capacity() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    assert!(set.capacity() >= 100);
    set.shrink_to(0);
    assert!(set.capacity() >= 2);
}

#[should_panic]
fn test_shrink_to_below_current_capacity() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(50);
    set.insert(1);
    set.insert(2);
    set.shrink_to(100);
}

