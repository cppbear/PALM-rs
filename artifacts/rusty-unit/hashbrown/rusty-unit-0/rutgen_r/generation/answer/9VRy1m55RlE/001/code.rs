// Answer 0

#[test]
fn test_shrink_to_fit_empty_set() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    let initial_capacity = set.capacity();
    set.shrink_to_fit();
    assert_eq!(set.capacity(), initial_capacity);
}

#[test]
fn test_shrink_to_fit_with_one_element() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    assert!(set.capacity() >= 100);
    set.shrink_to_fit();
    assert!(set.capacity() >= 1);
}

#[test]
fn test_shrink_to_fit_with_multiple_elements() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    set.insert(3);
    assert!(set.capacity() >= 100);
    set.shrink_to_fit();
    assert!(set.capacity() >= 3);
}

#[test]
fn test_shrink_to_fit_large_initial_capacity() {
    use hashbrown::HashSet;

    let mut set = HashSet::with_capacity(1000);
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    assert!(set.capacity() >= 1000);
    set.shrink_to_fit();
    assert!(set.capacity() >= 5);
}

