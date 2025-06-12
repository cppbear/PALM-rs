// Answer 0

#[test]
fn test_shift_take_value_present() {
    struct SimpleHash;

    impl BuildHasher for SimpleHash {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set: IndexSet<i32, SimpleHash> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: SimpleHash,
        },
    };
    
    set.insert(1);
    set.insert(2);
    set.insert(3);

    let result = set.shift_take(&2);
    assert_eq!(result, Some(2));
}

#[test]
fn test_shift_take_value_absent() {
    struct SimpleHash;

    impl BuildHasher for SimpleHash {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set: IndexSet<i32, SimpleHash> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: SimpleHash,
        },
    };
    
    set.insert(1);
    set.insert(2);
    set.insert(3);

    let result = set.shift_take(&4);
    assert_eq!(result, None);
}

#[test]
fn test_shift_take_multiple_elements() {
    struct SimpleHash;

    impl BuildHasher for SimpleHash {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set: IndexSet<i32, SimpleHash> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: SimpleHash,
        },
    };
    
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);

    let result = set.shift_take(&3);
    assert_eq!(result, Some(3));
    assert_eq!(set.contains(&3), false);
}

#[test]
fn test_shift_take_empty_set() {
    struct SimpleHash;

    impl BuildHasher for SimpleHash {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set: IndexSet<i32, SimpleHash> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: SimpleHash,
        },
    };

    let result = set.shift_take(&1);
    assert_eq!(result, None);
}

