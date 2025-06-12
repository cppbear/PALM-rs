// Answer 0

#[test]
fn test_hashset_with_hasher_default() {
    let hasher = DefaultHashBuilder::default();
    let set = HashSet::with_hasher(hasher);
}

#[test]
fn test_hashset_with_hasher_custom() {
    use std::collections::hash_map::RandomState;
    let hasher = RandomState::new();
    let set = HashSet::with_hasher(hasher);
}

#[test]
fn test_hashset_with_hasher_filled() {
    let hasher = DefaultHashBuilder::default();
    let mut set = HashSet::with_hasher(hasher);
    set.insert(1);
    set.insert(2);
}

#[test]
fn test_hashset_with_hasher_empty() {
    let hasher = DefaultHashBuilder::default();
    let set = HashSet::with_hasher(hasher);
}

#[test]
#[should_panic]
fn test_hashset_with_hasher_zero_capacity() {
    let hasher = DefaultHashBuilder::default();
    let set = HashSet::with_capacity_and_hasher(0, hasher);
}

