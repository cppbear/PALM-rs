// Answer 0

#[test]
fn test_with_capacity_zero() {
    let set: hashbrown::HashSet<i32> = hashbrown::HashSet::with_capacity(0);
    assert_eq!(set.capacity(), 0);
}

#[test]
fn test_with_capacity_ten() {
    let set: hashbrown::HashSet<i32> = hashbrown::HashSet::with_capacity(10);
    assert!(set.capacity() >= 10);
}

#[test]
fn test_with_capacity_one() {
    let set: hashbrown::HashSet<i32> = hashbrown::HashSet::with_capacity(1);
    assert!(set.capacity() >= 1);
}

#[test]
fn test_with_capacity_large() {
    let capacity = 1_000_000;
    let set: hashbrown::HashSet<i32> = hashbrown::HashSet::with_capacity(capacity);
    assert!(set.capacity() >= capacity);
}

