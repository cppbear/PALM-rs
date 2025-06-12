// Answer 0

#[test]
fn test_shrink_to_valid_min_capacity() {
    let mut set = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    assert!(set.allocation_size() >= 100);
    set.shrink_to(10);
    assert!(set.allocation_size() >= 10);
    set.shrink_to(0);
    assert!(set.allocation_size() >= 2);
}

#[should_panic]
fn test_shrink_to_panics_when_current_capacity_is_smaller() {
    let mut set = HashSet::with_capacity(5);
    set.insert(1);
    set.shrink_to(10); // Should panic as current capacity is less than 10
}

#[test]
fn test_shrink_to_exact_capacity() {
    let mut set = HashSet::with_capacity(20);
    for i in 0..15 {
        set.insert(i);
    }
    assert!(set.allocation_size() >= 20);
    set.shrink_to(15);
    assert!(set.allocation_size() >= 15);
}

#[test]
fn test_shrink_to_zero() {
    let mut set = HashSet::with_capacity(30);
    for i in 0..10 {
        set.insert(i);
    }
    assert!(set.allocation_size() >= 30);
    set.shrink_to(0);
    assert!(set.allocation_size() >= 10);    
}

