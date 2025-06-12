// Answer 0

#[test]
fn test_capacity_empty_set() {
    let set: std::collections::HashSet<i32> = std::collections::HashSet::new();
    assert_eq!(set.capacity(), 0);
}

#[test]
fn test_capacity_non_empty_set() {
    let mut set: std::collections::HashSet<i32> = std::collections::HashSet::with_capacity(10);
    set.insert(1);
    set.insert(2);
    set.insert(3);
    assert!(set.capacity() >= 10);
}

#[test]
fn test_capacity_large_set() {
    let mut set: std::collections::HashSet<i32> = std::collections::HashSet::with_capacity(100);
    for i in 0..100 {
        set.insert(i);
    }
    assert!(set.capacity() >= 100);
}

#[test]
fn test_capacity_after_removal() {
    let mut set: std::collections::HashSet<i32> = std::collections::HashSet::with_capacity(5);
    set.insert(1);
    set.insert(2);
    set.remove(&1);
    assert!(set.capacity() >= 5);
}

#[test]
fn test_capacity_after_growth() {
    let mut set: std::collections::HashSet<i32> = std::collections::HashSet::new();
    for i in 0..15 {
        set.insert(i);
    }
    assert!(set.capacity() >= 15);
}

