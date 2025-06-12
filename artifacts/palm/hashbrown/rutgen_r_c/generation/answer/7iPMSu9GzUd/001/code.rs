// Answer 0

#[test]
fn test_hashset_with_capacity_non_zero() {
    let capacity = 10;
    let set: HashSet<i32> = HashSet::with_capacity(capacity);
    assert!(set.map.table.capacity() >= capacity);
}

#[test]
fn test_hashset_with_capacity_zero() {
    let capacity = 0;
    let set: HashSet<i32> = HashSet::with_capacity(capacity);
    assert!(set.map.table.capacity() == 0);
}

#[test]
fn test_hashset_with_capacity_boundary() {
    let capacity = usize::MAX;
    let set: HashSet<i32> = HashSet::with_capacity(capacity);
    assert!(set.map.table.capacity() >= capacity);
}

