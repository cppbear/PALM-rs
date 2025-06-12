// Answer 0

#[test]
fn test_hashset_with_capacity_and_hasher_nonzero() {
    use hashbrown::{HashSet, DefaultHashBuilder};
    
    let hasher = DefaultHashBuilder::default();
    let capacity = 10;
    let set: HashSet<i32, DefaultHashBuilder> = HashSet::with_capacity_and_hasher(capacity, hasher);
    
    assert_eq!(set.map.table.len(), 0); // ensure set is empty
}

#[test]
fn test_hashset_with_capacity_and_hasher_zero() {
    use hashbrown::{HashSet, DefaultHashBuilder};

    let hasher = DefaultHashBuilder::default();
    let capacity = 0;
    let set: HashSet<i32, DefaultHashBuilder> = HashSet::with_capacity_and_hasher(capacity, hasher);
    
    assert_eq!(set.map.table.len(), 0); // ensure set is empty
}

#[test]
fn test_hashset_with_capacity_and_hasher_capacity_increases() {
    use hashbrown::{HashSet, DefaultHashBuilder};

    let hasher = DefaultHashBuilder::default();
    let capacity = 5;
    let mut set: HashSet<i32, DefaultHashBuilder> = HashSet::with_capacity_and_hasher(capacity, hasher);
    
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    assert_eq!(set.map.table.len(), 3); // ensure all inserted elements are counted
}

#[test]
#[should_panic]
fn test_hashset_with_capacity_and_hasher_negative_capacity() {
    use hashbrown::{HashSet, DefaultHashBuilder};

    let hasher = DefaultHashBuilder::default();
    
    let _set: HashSet<i32, DefaultHashBuilder> = HashSet::with_capacity_and_hasher(usize::MAX, hasher); // Edge case, should panic or fail with bad capacity.
}

