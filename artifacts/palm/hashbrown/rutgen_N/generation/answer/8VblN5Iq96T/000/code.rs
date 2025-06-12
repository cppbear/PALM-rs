// Answer 0

#[test]
fn test_hasher() {
    use hashbrown::HashSet;
    use hashbrown::DefaultHashBuilder;

    let hasher = DefaultHashBuilder::default();
    let set: HashSet<i32> = HashSet::with_hasher(hasher);
    let hasher_ref: &DefaultHashBuilder = set.hasher();

    assert!(hasher_ref as *const _ == set.map.hasher() as *const _);
}

#[test]
fn test_hasher_with_elements() {
    use hashbrown::HashSet;
    use hashbrown::DefaultHashBuilder;

    let hasher = DefaultHashBuilder::default();
    let mut set: HashSet<i32> = HashSet::with_hasher(hasher);
    set.insert(1);
    set.insert(2);

    let hasher_ref: &DefaultHashBuilder = set.hasher();

    assert!(hasher_ref as *const _ == set.map.hasher() as *const _);
}

#[test]
fn test_hasher_empty_set() {
    use hashbrown::HashSet;
    use hashbrown::DefaultHashBuilder;

    let hasher = DefaultHashBuilder::default();
    let set: HashSet<i32> = HashSet::with_hasher(hasher);

    let hasher_ref: &DefaultHashBuilder = set.hasher();
    
    assert!(hasher_ref as *const _ == set.map.hasher() as *const _);
}

