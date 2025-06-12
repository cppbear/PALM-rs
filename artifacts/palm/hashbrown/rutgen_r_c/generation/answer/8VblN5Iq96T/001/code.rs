// Answer 0

#[test]
fn test_hasher_with_default_hash_builder() {
    use hashbrown::{HashSet, DefaultHashBuilder};

    let hasher = DefaultHashBuilder::default();
    let set: HashSet<i32, DefaultHashBuilder> = HashSet::with_hasher_in(hasher, Global);
    
    let retrieved_hasher: &DefaultHashBuilder = set.hasher();
    assert_eq!(retrieved_hasher as *const _, hasher as *const _);
}

#[test]
fn test_hasher_with_custom_hash_builder() {
    use hashbrown::{HashSet, DefaultHashBuilder};
    use std::collections::hash_map::RandomState;

    let custom_hasher = RandomState::new();
    let set: HashSet<i32, RandomState> = HashSet::with_hasher_in(custom_hasher.clone(), Global);
    
    let retrieved_hasher: &RandomState = set.hasher();
    assert_eq!(retrieved_hasher as *const _, custom_hasher as *const _);
}

