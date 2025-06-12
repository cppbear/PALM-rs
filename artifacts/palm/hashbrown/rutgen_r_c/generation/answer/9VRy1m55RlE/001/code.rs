// Answer 0

#[test]
fn test_shrink_to_fit_empty() {
    let mut set: HashSet<i32> = HashSet::with_capacity(0);
    set.shrink_to_fit();
    assert!(set.allocation_size() == 0);
}

#[test]
fn test_shrink_to_fit_non_empty() {
    let mut set: HashSet<i32> = HashSet::with_capacity(10);
    set.insert(1);
    set.insert(2);
    assert!(set.allocation_size() >= 10);
    set.shrink_to_fit();
    assert!(set.allocation_size() >= 2);
}

#[test]
fn test_shrink_to_fit_single_element() {
    let mut set: HashSet<i32> = HashSet::with_capacity(5);
    set.insert(42);
    assert!(set.allocation_size() >= 5);
    set.shrink_to_fit();
    assert!(set.allocation_size() >= 1);
}

#[test]
fn test_shrink_to_fit_multiple_elements() {
    let mut set: HashSet<i32> = HashSet::with_capacity(20);
    for i in 0..15 {
        set.insert(i);
    }
    assert!(set.allocation_size() >= 20);
    set.shrink_to_fit();
    assert!(set.allocation_size() >= 15);
}

