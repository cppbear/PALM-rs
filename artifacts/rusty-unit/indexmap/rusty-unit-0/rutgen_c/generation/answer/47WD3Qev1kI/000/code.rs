// Answer 0

#[test]
fn test_insert_before_at_start() {
    struct SimpleHasher;
    
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: super::IndexSet<char, SimpleHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: SimpleHasher,
        },
    };

    assert_eq!(set.insert_before(0, 'a'), (0, true));
    assert_eq!(set.insert_before(0, 'b'), (0, true));
    assert_eq!(set.insert_before(1, 'c'), (1, true));
}

#[test]
#[should_panic]
fn test_insert_before_out_of_bounds() {
    struct SimpleHasher;

    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: super::IndexSet<char, SimpleHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: SimpleHasher,
        },
    };

    set.insert_before(1, 'a'); // Will panic as the index is out of bounds.
}

#[test]
fn test_insert_before_at_end() {
    struct SimpleHasher;

    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: super::IndexSet<char, SimpleHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: SimpleHasher,
        },
    };

    set.insert_before(0, 'a');
    set.insert_before(1, 'b');

    assert_eq!(set.insert_before(2, 'c'), (2, true));
    assert_eq!(set.insert_before(2, 'b'), (1, false));
}

#[test]
fn test_insert_before_shift_existing() {
    struct SimpleHasher;

    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: super::IndexSet<char, SimpleHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: SimpleHasher,
        },
    };

    set.insert_before(0, 'a');
    set.insert_before(1, 'b');
    
    assert_eq!(set.insert_before(1, 'a'), (0, false)); // 'a' is already there
    assert_eq!(set.insert_before(1, 'c'), (1, true)); // Shift 'b' down
}

