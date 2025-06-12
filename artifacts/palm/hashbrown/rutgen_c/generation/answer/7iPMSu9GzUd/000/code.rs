// Answer 0

#[test]
fn test_hash_set_with_capacity_non_zero() {
    let set: HashSet<i32> = HashSet::with_capacity(10);
    assert!(set.map.table.capacity() >= 10);
}

#[test]
fn test_hash_set_with_capacity_zero() {
    let set: HashSet<i32> = HashSet::with_capacity(0);
    assert_eq!(set.map.table.capacity(), 0);
}

