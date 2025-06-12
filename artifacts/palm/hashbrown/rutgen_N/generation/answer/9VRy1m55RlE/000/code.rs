// Answer 0

#[test]
fn test_shrink_to_fit_no_elements() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::with_capacity(100);
    assert!(set.capacity() >= 100);
    set.shrink_to_fit();
    assert!(set.capacity() >= 0); // After shrinking, capacity can be zero since no elements are in it
}

#[test]
fn test_shrink_to_fit_multiple_elements() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    assert!(set.capacity() >= 100);
    set.shrink_to_fit();
    assert!(set.capacity() >= 2); // Capacity should be reduced but still accommodate inserted elements
}

#[test]
fn test_shrink_to_fit_one_element() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    assert!(set.capacity() >= 100);
    set.shrink_to_fit();
    assert!(set.capacity() >= 1); // After shrinking, the capacity should be at least one
}

#[test]
fn test_shrink_to_fit_capacity_change() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(200);
    set.insert(1);
    set.insert(2);
    set.insert(3);
    assert!(set.capacity() >= 200);
    set.shrink_to_fit();
    assert!(set.capacity() >= 3); // Capacity should be adjusted down to at least the number of elements
}

