// Answer 0

#[test]
fn test_contains_existing_value() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    struct TestIndexSet {
        map: super::IndexMap<u32, (), TestHasher>,
    }

    impl TestIndexSet {
        pub fn new() -> Self {
            Self {
                map: super::IndexMap::new(),
            }
        }
        
        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            Q: ?Sized + Hash + super::Equivalent<u32>,
        {
            self.map.contains_key(value)
        }
    }

    let mut set = TestIndexSet::new();
    set.map.insert(1, ());
    assert!(set.contains(&1));
}

#[test]
fn test_contains_non_existing_value() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    struct TestIndexSet {
        map: super::IndexMap<u32, (), TestHasher>,
    }

    impl TestIndexSet {
        pub fn new() -> Self {
            Self {
                map: super::IndexMap::new(),
            }
        }
        
        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            Q: ?Sized + Hash + super::Equivalent<u32>,
        {
            self.map.contains_key(value)
        }
    }

    let set = TestIndexSet::new();
    assert!(!set.contains(&1));
}

#[test]
fn test_contains_empty_set() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    struct TestIndexSet {
        map: super::IndexMap<u32, (), TestHasher>,
    }

    impl TestIndexSet {
        pub fn new() -> Self {
            Self {
                map: super::IndexMap::new(),
            }
        }
        
        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            Q: ?Sized + Hash + super::Equivalent<u32>,
        {
            self.map.contains_key(value)
        }
    }

    let set = TestIndexSet::new();
    assert!(!set.contains(&0));
}

