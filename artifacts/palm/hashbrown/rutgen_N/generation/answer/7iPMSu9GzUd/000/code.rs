// Answer 0

#[test]
fn test_with_capacity_zero() {
    let set: hashbrown::HashSet<i32> = hashbrown::HashSet::with_capacity(0);
    assert_eq!(set.capacity(), 0);
}

#[test]
fn test_with_capacity_non_zero() {
    let capacity = 10;
    let set: hashbrown::HashSet<i32> = hashbrown::HashSet::with_capacity(capacity);
    assert!(set.capacity() >= capacity);
}

#[test]
fn test_with_capacity_large_number() {
    let capacity = 1_000_000;
    let set: hashbrown::HashSet<i32> = hashbrown::HashSet::with_capacity(capacity);
    assert!(set.capacity() >= capacity);
}

