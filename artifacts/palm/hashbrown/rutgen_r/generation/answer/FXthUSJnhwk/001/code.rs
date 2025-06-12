// Answer 0

#[test]
fn test_with_hasher_default() {
    use hashbrown::{HashSet, DefaultHashBuilder};

    let hasher = DefaultHashBuilder::default();
    let set: HashSet<i32> = HashSet::with_hasher(hasher);
    
    assert!(set.is_empty());
}

#[test]
fn test_with_hasher_custom() {
    use hashbrown::{HashSet, RandomState};

    let hasher = RandomState::new();
    let set: HashSet<i32> = HashSet::with_hasher(hasher);
    
    assert!(set.is_empty());
}

#[test]
fn test_with_hasher_insert() {
    use hashbrown::{HashSet, DefaultHashBuilder};

    let hasher = DefaultHashBuilder::default();
    let mut set: HashSet<i32> = HashSet::with_hasher(hasher);
    
    set.insert(1);
    set.insert(2);
    
    assert_eq!(set.len(), 2);
}

#[test]
fn test_with_hasher_no_initialization() {
    use hashbrown::{HashSet, RandomState};

    let hasher = RandomState::new();
    let set: HashSet<String> = HashSet::with_hasher(hasher);
    
    assert!(set.is_empty());
}

#[should_panic]
fn test_with_hasher_panic_on_invalid_hasher() {
    struct InvalidHasher;

    impl std::hash::BuildHasher for InvalidHasher {
        type Hasher = ();

        fn build_hasher(&self) -> Self::Hasher {
            ()
        }
    }

    use hashbrown::HashSet;

    let hasher = InvalidHasher;
    let _set: HashSet<i32> = HashSet::with_hasher(hasher);
}

