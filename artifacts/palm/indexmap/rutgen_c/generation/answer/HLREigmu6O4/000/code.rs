// Answer 0

#[test]
fn test_replace_full_existing_value() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }
    
    let mut set: IndexSet<u32, DummyHasher> = IndexSet {
        map: IndexMap {
            core: IndexMapCore {
                indices: Indices::default(),
                entries: Entries::default(),
            },
            hash_builder: DummyHasher,
        },
    };

    set.insert(1);
    let (index, replaced) = set.replace_full(1);
    assert_eq!(index, 0);
    assert_eq!(replaced, None);
}

#[test]
fn test_replace_full_no_existing_value() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }
    
    let mut set: IndexSet<u32, DummyHasher> = IndexSet {
        map: IndexMap {
            core: IndexMapCore {
                indices: Indices::default(),
                entries: Entries::default(),
            },
            hash_builder: DummyHasher,
        },
    };

    let (index, replaced) = set.replace_full(2);
    assert_eq!(index, 0);
    assert_eq!(replaced, None);
}

#[test]
fn test_replace_full_replacing_value() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }
  
    let mut set: IndexSet<u32, DummyHasher> = IndexSet {
        map: IndexMap {
            core: IndexMapCore {
                indices: Indices::default(),
                entries: Entries::default(),
            },
            hash_builder: DummyHasher,
        },
    };

    set.insert(3);
    let (index, replaced) = set.replace_full(3);
    assert_eq!(index, 0);
    assert_eq!(replaced, None); // No previous value to replace
    
    let (index, replaced) = set.replace_full(3);
    assert_eq!(index, 0);
    assert_eq!(replaced, Some(3)); // Replaces with itself, should not replace anything else
}

