// Answer 0

#[test]
fn test_with_default_hasher() {
    use hashbrown::HashSet;
    use hashbrown::DefaultHashBuilder;

    let hasher = DefaultHashBuilder::default();
    let mut set = HashSet::with_hasher(hasher);
    set.insert(1);
    set.insert(2);
    
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(!set.contains(&3));
}

#[test]
fn test_with_custom_hasher() {
    use hashbrown::HashSet;
    use hashbrown::DefaultHashBuilder;

    let hasher = DefaultHashBuilder::default();
    let mut set = HashSet::with_hasher(hasher);
    set.insert(42);
    
    assert!(set.contains(&42));
    assert!(!set.contains(&100));
}

#[test]
fn test_with_empty_set() {
    use hashbrown::HashSet;
    use hashbrown::DefaultHashBuilder;

    let hasher = DefaultHashBuilder::default();
    let set: HashSet<u32> = HashSet::with_hasher(hasher);
    
    assert!(set.is_empty());
}

#[test]
fn test_hashed_insert() {
    use std::collections::hash_map::RandomState;
    use hashbrown::HashSet;

    let hasher = RandomState::new();
    let mut set = HashSet::with_hasher(hasher);
    
    set.insert(10);
    set.insert(20);
    
    assert!(set.contains(&10));
    assert!(set.contains(&20));
    assert!(!set.contains(&30));
}

