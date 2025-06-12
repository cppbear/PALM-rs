// Answer 0

#[test]
fn test_hashset_with_zero_capacity() {
    let hasher = DefaultHashBuilder::default();
    let allocator = Global;
    let set: HashSet<i32, _, _> = HashSet::with_capacity_and_hasher_in(0, hasher, allocator);
    assert_eq!(set.map.table.len(), 0);
}

#[test]
fn test_hashset_with_non_zero_capacity() {
    let hasher = DefaultHashBuilder::default();
    let allocator = Global;
    let set: HashSet<i32, _, _> = HashSet::with_capacity_and_hasher_in(10, hasher, allocator);
    assert_eq!(set.map.table.len(), 0); // Table starts empty
}

#[test]
fn test_hashset_inserts() {
    let hasher = DefaultHashBuilder::default();
    let allocator = Global;
    let mut set: HashSet<i32, _, _> = HashSet::with_capacity_and_hasher_in(10, hasher, allocator);
    set.map.insert(1, ());
    set.map.insert(2, ());
    assert_eq!(set.map.table.len(), 2);
}

#[should_panic]
fn test_hashset_with_negative_capacity() {
    let hasher = DefaultHashBuilder::default();
    let allocator = Global;
    let _set: HashSet<i32, _, _> = HashSet::with_capacity_and_hasher_in(usize::MAX, hasher, allocator);
}

