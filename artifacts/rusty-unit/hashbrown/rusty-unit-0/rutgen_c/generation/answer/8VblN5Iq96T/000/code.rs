// Answer 0

#[test]
fn test_hashset_hasher_default() {
    use hashbrown::{HashSet, DefaultHashBuilder};

    let hasher = DefaultHashBuilder::default();
    let set: HashSet<i32, DefaultHashBuilder> = HashSet::with_hasher_in(hasher.clone(), Global);
    let hasher_ref: &DefaultHashBuilder = set.hasher();
    
    assert_eq!(std::ptr::addr_of!(*hasher_ref), std::ptr::addr_of!(hasher));
}

#[test]
fn test_hashset_hasher_with_capacity() {
    use hashbrown::{HashSet, DefaultHashBuilder};

    let hasher = DefaultHashBuilder::default();
    let set: HashSet<i32, DefaultHashBuilder> = HashSet::with_capacity_and_hasher_in(10, hasher.clone(), Global);
    let hasher_ref: &DefaultHashBuilder = set.hasher();
    
    assert_eq!(std::ptr::addr_of!(*hasher_ref), std::ptr::addr_of!(hasher));
}

