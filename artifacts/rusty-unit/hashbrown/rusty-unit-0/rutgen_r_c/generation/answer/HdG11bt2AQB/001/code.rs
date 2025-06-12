// Answer 0

#[test]
fn test_capacity_empty_set() {
    let set: HashSet<i32> = HashSet::with_capacity(0);
    assert_eq!(set.capacity(), 0);
}

#[test]
fn test_capacity_with_positive_value() {
    let set: HashSet<i32> = HashSet::with_capacity(100);
    assert!(set.capacity() >= 100);
}

#[test]
fn test_capacity_with_exact_value() {
    let set: HashSet<i32> = HashSet::with_capacity(50);
    assert_eq!(set.capacity(), 50);
}

#[test]
fn test_capacity_with_large_value() {
    let set: HashSet<i32> = HashSet::with_capacity(1_000_000);
    assert!(set.capacity() >= 1_000_000);
}

#[test]
fn test_capacity_with_small_value() {
    let set: HashSet<i32> = HashSet::with_capacity(1);
    assert!(set.capacity() >= 1);
}

