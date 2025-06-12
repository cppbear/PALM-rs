// Answer 0

#[test]
fn test_replace_existing_value() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: super::IndexSet<i32, SimpleHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: SimpleHasher,
        },
    };

    set.insert(1);
    set.insert(2);
    let replaced_value = set.replace(1);
    assert_eq!(replaced_value, Some(1));
}

#[test]
fn test_replace_non_existing_value() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: super::IndexSet<i32, SimpleHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: SimpleHasher,
        },
    };

    let replaced_value = set.replace(3);
    assert_eq!(replaced_value, None);
}

#[test]
fn test_replace_multiple_values() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: super::IndexSet<i32, SimpleHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: SimpleHasher,
        },
    };

    set.insert(1);
    set.insert(2);
    set.insert(3);
    let replaced_value = set.replace(2);
    assert_eq!(replaced_value, Some(2));
    
    // Verify that the other values are unchanged
    assert_eq!(set.replace(1), Some(1));
    assert_eq!(set.replace(4), None);
}

